use alloc::string::{String, ToString};
use alloc::vec::Vec;

use sentc_crypto_light::StdUserPreVerifyLogin;
use sentc_crypto_utils::error::SdkUtilError;
use sentc_crypto_utils::http::{auth_req, make_req, non_auth_req, HttpMethod};
use sentc_crypto_utils::{handle_general_server_response, handle_server_response};
use serde::{Deserialize, Serialize};
use server_dashboard_common::customer::{
	CustomerData,
	CustomerDataOutput,
	CustomerDoneRegistrationInput,
	CustomerRegisterData,
	CustomerRegisterOutput,
	CustomerUpdateInput,
};
use server_dashboard_common::sdk_common::user::{
	CaptchaCreateOutput,
	CaptchaInput,
	DoneLoginLightServerOutput,
	OtpRecoveryKeysOutput,
	OtpRegister,
	UserDeviceRegisterInput,
};
use server_dashboard_common::sdk_common::{DeviceId, UserId};

use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct CustomerVerifyLoginOutput
{
	pub email_data: CustomerDataOutput,
	pub jwt: String,
	pub refresh_token: String,
	pub user_id: UserId,
	pub device_id: DeviceId,
}

pub struct PrepareLoginOtpOutput
{
	pub master_key: String,
	pub auth_key: String,
}

#[allow(clippy::large_enum_variant)]
pub enum PreLoginOut
{
	Direct(CustomerVerifyLoginOutput),
	Otp(PrepareLoginOtpOutput),
}

pub async fn captcha_req(base_url: String) -> Result<CaptchaCreateOutput, String>
{
	let url = base_url + "/api/v1/customer/captcha";

	let res = non_auth_req(HttpMethod::GET, url.as_str(), "", None).await?;

	let out: CaptchaCreateOutput = handle_server_response(res.as_str())?;

	Ok(out)
}

#[allow(clippy::too_many_arguments)]
pub async fn register(
	base_url: String,
	email: String,
	password: &str,
	name: String,
	first_name: String,
	company: Option<String>,
	captcha_solution: String,
	captcha_id: String,
) -> Result<String, String>
{
	let register_data = sentc_crypto_light::user::register(email.as_str(), password)?;
	let register_data: UserDeviceRegisterInput = serde_json::from_str(&register_data).map_err(SdkUtilError::JsonParseFailed)?;

	let input = CustomerRegisterData {
		customer_data: CustomerData {
			name,
			first_name,
			company,
		},
		email,
		register_data,
		captcha_input: CaptchaInput {
			captcha_solution,
			captcha_id,
		},
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/register";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

	let out: CustomerRegisterOutput = handle_server_response(res.as_str())?;

	Ok(out.customer_id)
}

pub async fn done_register(base_url: String, jwt: &str, token: String) -> Result<(), String>
{
	//call this for update email too

	let input = CustomerDoneRegistrationInput {
		token,
	};

	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/register_validation";

	let res = make_req(HttpMethod::POST, url.as_str(), "", Some(input), Some(jwt), None).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn refresh_jwt(base_url: String, old_jwt: &str, refresh_token: String) -> Result<String, String>
{
	let url = base_url + "/api/v1/customer/refresh";

	let input = sentc_crypto_light::user::prepare_refresh_jwt(refresh_token)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(old_jwt), None).await?;

	let server_output: DoneLoginLightServerOutput = handle_server_response(res.as_str())?;

	Ok(server_output.jwt)
}

async fn prepare_user_fresh_jwt(
	base_url: String,
	user_identifier: &str,
	password: &str,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<
	(
		String,
		StdUserPreVerifyLogin,
		sentc_crypto_light::sdk_common::user::DoneLoginServerOutput,
	),
	String,
>
{
	//first make the prep login req to get the output
	let prep_login_out = prepare_login_start(base_url.clone(), user_identifier).await?;

	let (keys, done_login_out) = done_login_internally(
		base_url,
		user_identifier,
		password,
		&prep_login_out,
		mfa_token,
		mfa_recovery,
	)
	.await?;

	Ok((prep_login_out, keys, done_login_out))
}

pub async fn get_fresh_jwt(
	base_url: String,
	user_identifier: &str,
	password: &str,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<String, String>
{
	let (_, keys, _) = prepare_user_fresh_jwt(base_url.clone(), user_identifier, password, mfa_token, mfa_recovery).await?;

	let keys = verify_login(base_url, keys).await?;

	Ok(keys.jwt)
}

async fn verify_login(base_url: String, pre_verify: StdUserPreVerifyLogin) -> Result<CustomerVerifyLoginOutput, String>
{
	let url = base_url + "/api/v1/customer/verify_login";
	let server_out = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(pre_verify.challenge)).await?;

	let out: server_dashboard_common::customer::CustomerDoneLoginOutput = handle_server_response(&server_out)?;

	Ok(CustomerVerifyLoginOutput {
		email_data: out.email_data,
		jwt: out.verify.jwt,
		refresh_token: out.verify.refresh_token,
		user_id: pre_verify.user_id,
		device_id: pre_verify.device_id,
	})
}

//__________________________________________________________________________________________________

async fn prepare_login_start(base_url: String, user_identifier: &str) -> Result<String, String>
{
	let user_id_input = sentc_crypto_light::user::prepare_login_start(user_identifier)?;

	let url = base_url + "/api/v1/customer/prepare_login";

	let res = non_auth_req(HttpMethod::POST, &url, "", Some(user_id_input)).await?;

	Ok(res)
}

async fn done_login_internally(
	base_url: String,
	user_identifier: &str,
	password: &str,
	prepare_login_res: &str,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<
	(
		StdUserPreVerifyLogin,
		sentc_crypto_light::sdk_common::user::DoneLoginServerOutput,
	),
	String,
>
{
	let (input, auth_key, master_key) = sentc_crypto_light::user::prepare_login(user_identifier, password, prepare_login_res)?;

	let url = base_url.clone() + "/api/v1/customer/done_login";
	let server_out = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

	match sentc_crypto_light::user::check_done_login(&server_out)? {
		sentc_crypto_light::sdk_common::user::DoneLoginServerReturn::Direct(d) => {
			let out = sentc_crypto_light::user::done_login(&master_key, auth_key, user_identifier.to_string(), d.clone())?;

			Ok((out, d))
		},
		sentc_crypto_light::sdk_common::user::DoneLoginServerReturn::Otp => {
			//if user enables mfa it must be saved in the user data, so the token is needed before doing the req
			let mfa_token = mfa_token.ok_or(SdkUtilError::JsonToStringFailed)?;
			let mfa_recovery = mfa_recovery.ok_or(SdkUtilError::JsonToStringFailed)?;

			//use this with the token of the auth app but without to verify

			let url = base_url.clone() +
				if mfa_recovery {
					"/api/v1/customer/validate_recovery_otp"
				} else {
					"/api/v1/customer/validate_mfa"
				};

			let input = sentc_crypto_light::user::prepare_validate_mfa(auth_key.clone(), user_identifier.to_string(), mfa_token)?;

			let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

			let d: sentc_crypto_light::sdk_common::user::DoneLoginServerOutput = handle_server_response(&res)?;

			let out = sentc_crypto_light::user::done_login(&master_key, auth_key, user_identifier.to_string(), d.clone())?;

			Ok((out, d))
		},
	}
}

pub async fn login(base_url: String, email: &str, password: &str) -> Result<PreLoginOut, String>
{
	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let prep_server_input = sentc_crypto_light::user::prepare_login_start(email)?;

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (input, auth_key, derived_master_key) = sentc_crypto_light::user::prepare_login(email, password, res.as_str())?;

	let url = base_url.clone() + "/api/v1/customer/done_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

	match sentc_crypto_light::user::check_done_login(&res)? {
		sentc_crypto_light::sdk_common::user::DoneLoginServerReturn::Direct(d) => {
			let keys = sentc_crypto_light::user::done_login(&derived_master_key, auth_key, email.to_string(), d).unwrap();

			let out = verify_login(base_url, keys).await?;

			Ok(PreLoginOut::Direct(out))
		},
		sentc_crypto_light::sdk_common::user::DoneLoginServerReturn::Otp => {
			let master_key: sentc_crypto_light::sdk_keys::util::MasterKeyFormat = derived_master_key.into();

			Ok(PreLoginOut::Otp(PrepareLoginOtpOutput {
				master_key: master_key.to_string()?,
				auth_key,
			}))
		},
	}
}

pub async fn mfa_login(
	base_url: String,
	master_key_encryption: &str,
	auth_key: String,
	user_identifier: String,
	token: String,
	recovery: bool,
) -> Result<CustomerVerifyLoginOutput, String>
{
	let url = base_url.clone() +
		if recovery {
			"/api/v1/customer/validate_recovery_otp"
		} else {
			"/api/v1/customer/validate_mfa"
		};

	let input = sentc_crypto_light::user::prepare_validate_mfa(auth_key.clone(), user_identifier.clone(), token)?;

	let res = non_auth_req(HttpMethod::POST, &url, "", Some(input)).await?;

	let keys = sentc_crypto_light::user::done_validate_mfa(master_key_encryption, auth_key, user_identifier, &res)?;

	verify_login(base_url, keys).await
}

pub async fn update(base_url: String, jwt: &str, new_email: String) -> Result<(), String>
{
	let update_data = CustomerUpdateInput {
		new_email,
	};
	let update_data = utils::to_string(&update_data)?;

	let url = base_url + "/api/v1/customer";

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(update_data), Some(jwt), None).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn update_data(base_url: String, jwt: &str, name: String, first_name: String, company: Option<String>) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/data";

	let input = CustomerData {
		name,
		first_name,
		company,
	};
	let input = utils::to_string(&input)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt), None).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn delete_customer(base_url: String, fresh_jwt: &str) -> Result<(), String>
{
	//get now the fresh jwt from the jwt fn

	let url = base_url + "/api/v1/customer";

	let res = make_req(HttpMethod::DELETE, url.as_str(), "", None, Some(fresh_jwt), None).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn prepare_reset_password(base_url: String, email: String, captcha_solution: String, captcha_id: String) -> Result<(), String>
{
	let input = server_dashboard_common::customer::CustomerResetPasswordInput {
		email,
		captcha_input: CaptchaInput {
			captcha_solution,
			captcha_id,
		},
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/password_reset";

	let res = non_auth_req(HttpMethod::PUT, url.as_str(), "", Some(input)).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn done_reset_password(base_url: String, token: String, email: &str, new_pw: &str) -> Result<(), String>
{
	//call this fn from the email token link

	let reset_password_data = sentc_crypto_light::user::register(email, new_pw)?;
	let reset_password_data: UserDeviceRegisterInput = serde_json::from_str(&reset_password_data).map_err(SdkUtilError::JsonParseFailed)?;

	let input = server_dashboard_common::customer::CustomerDonePasswordResetInput {
		token, //token from the email
		reset_password_data,
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/password_reset_validation";

	let res = non_auth_req(HttpMethod::PUT, url.as_str(), "", Some(input)).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn change_password(
	base_url: String,
	email: &str,
	old_pw: &str,
	new_pw: &str,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<(), String>
{
	let (prep_login_out, keys, done_login_out) = prepare_user_fresh_jwt(base_url.clone(), email, old_pw, mfa_token, mfa_recovery).await?;

	let keys = verify_login(base_url.clone(), keys).await?;

	let input = sentc_crypto_light::user::change_password(old_pw, new_pw, &prep_login_out, done_login_out)?;

	//now change the pw
	let url = base_url + "/api/v1/customer/password";

	//keys.jwt is the fresh jwt
	let res = make_req(HttpMethod::PUT, &url, "", Some(input), Some(&keys.jwt), None).await?;

	Ok(handle_general_server_response(&res)?)
}

//__________________________________________________________________________________________________
//Otp
fn create_otp_url(issuer: &str, audience: &str, secret: &str) -> String
{
	"otpauth://totp/".to_string() + issuer + ":" + audience + "?secret=" + secret + "&algorithm=SHA256&issuer=" + issuer
}

async fn register_raw_otp(base_url: String, jwt: &str) -> Result<OtpRegister, String>
{
	let url = base_url + "/api/v1/customer/register_otp";

	let res = auth_req(HttpMethod::PATCH, &url, "", None, jwt).await?;

	Ok(handle_server_response(&res)?)
}

pub async fn register_otp(base_url: String, issuer: &str, audience: &str, fresh_jwt: &str) -> Result<(String, Vec<String>), String>
{
	let out = register_raw_otp(base_url, fresh_jwt).await?;

	Ok((create_otp_url(issuer, audience, &out.secret), out.recover))
}

pub async fn get_otp_recover_keys(base_url: String, jwt: &str) -> Result<OtpRecoveryKeysOutput, String>
{
	let url = base_url + "/api/v1/customer/otp_recovery_keys";

	let res = auth_req(HttpMethod::GET, &url, "", None, jwt).await?;

	Ok(handle_server_response(&res)?)
}

async fn reset_raw_otp(base_url: String, jwt: &str) -> Result<OtpRegister, String>
{
	let url = base_url + "/api/v1/customer/reset_otp";

	let res = auth_req(HttpMethod::PATCH, &url, "", None, jwt).await?;

	Ok(handle_server_response(&res)?)
}

pub async fn reset_otp(base_url: String, issuer: &str, audience: &str, fresh_jwt: &str) -> Result<(String, Vec<String>), String>
{
	let out = reset_raw_otp(base_url, fresh_jwt).await?;

	Ok((create_otp_url(issuer, audience, &out.secret), out.recover))
}

pub async fn disable_otp(base_url: String, fresh_jwt: &str) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/disable_otp";

	let res = auth_req(HttpMethod::PATCH, &url, "", None, fresh_jwt).await?;

	Ok(handle_general_server_response(&res)?)
}
