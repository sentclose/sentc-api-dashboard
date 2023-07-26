use alloc::string::{String, ToString};

use sentc_crypto_utils::error::SdkUtilError;
use sentc_crypto_utils::http::{make_req, non_auth_req, HttpMethod};
use sentc_crypto_utils::{handle_general_server_response, handle_server_response};
use serde::{Deserialize, Serialize};
use server_api_common::customer::{
	CustomerData,
	CustomerDataOutput,
	CustomerDoneRegistrationInput,
	CustomerRegisterData,
	CustomerRegisterOutput,
	CustomerUpdateInput,
};
use server_api_common::sdk_common::user::{CaptchaCreateOutput, CaptchaInput, DoneLoginLightServerOutput, UserDeviceRegisterInput};
use server_api_common::sdk_common::{DeviceId, UserId};

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

pub async fn login(base_url: String, email: &str, password: &str) -> Result<CustomerVerifyLoginOutput, String>
{
	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let prep_server_input = sentc_crypto_light::user::prepare_login_start(email)?;

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (input, auth_key, derived_master_key) = sentc_crypto_light::user::prepare_login(email, password, res.as_str())?;

	let url = base_url.clone() + "/api/v1/customer/done_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

	let keys = sentc_crypto_light::user::done_login(&derived_master_key, auth_key, email.to_string(), &res)?;

	let url = base_url + "/api/v1/customer/verify_login";
	let server_out = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(keys.challenge)).await?;

	let out: server_api_common::customer::CustomerDoneLoginOutput = handle_server_response(&server_out)?;

	Ok(CustomerVerifyLoginOutput {
		email_data: out.email_data,
		jwt: out.verify.jwt,
		refresh_token: out.verify.refresh_token,
		user_id: keys.user_id,
		device_id: keys.device_id,
	})
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

pub async fn delete_customer(base_url: String, email: &str, pw: &str) -> Result<(), String>
{
	let prep_server_input = sentc_crypto_light::user::prepare_login_start(email)?;

	//get a fresh jwt
	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (input, auth_key, derived_master_key) = sentc_crypto_light::user::prepare_login(email, pw, res.as_str())?;

	let url = base_url.clone() + "/api/v1/customer/done_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

	let keys = sentc_crypto_light::user::done_login(&derived_master_key, auth_key, email.to_string(), &res)?;

	let url = base_url.clone() + "/api/v1/customer/verify_login";
	let server_out = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(keys.challenge)).await?;

	let out: server_api_common::customer::CustomerDoneLoginOutput = handle_server_response(&server_out)?;

	let fresh_jwt = out.verify.jwt;

	let url = base_url + "/api/v1/customer";

	let res = make_req(
		HttpMethod::DELETE,
		url.as_str(),
		"",
		None,
		Some(fresh_jwt.as_str()),
		None,
	)
	.await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn prepare_reset_password(base_url: String, email: String, captcha_solution: String, captcha_id: String) -> Result<(), String>
{
	let input = server_api_common::customer::CustomerResetPasswordInput {
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

	let input = server_api_common::customer::CustomerDonePasswordResetInput {
		token, //token from the email
		reset_password_data,
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/password_reset_validation";

	let res = non_auth_req(HttpMethod::PUT, url.as_str(), "", Some(input)).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn change_password(base_url: String, email: &str, old_pw: &str, new_pw: &str) -> Result<(), String>
{
	let prep_server_input = sentc_crypto_light::user::prepare_login_start(email)?;

	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (input, auth_key, derived_master_key) = sentc_crypto_light::user::prepare_login(email, old_pw, res.as_str())?;

	let url = base_url.clone() + "/api/v1/customer/done_login";

	let res2 = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(input)).await?;

	let keys = sentc_crypto_light::user::done_login(&derived_master_key, auth_key, email.to_string(), &res2)?;

	let url = base_url.clone() + "/api/v1/customer/verify_login";
	let server_out = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(keys.challenge)).await?;

	let out: server_api_common::customer::CustomerDoneLoginOutput = handle_server_response(&server_out)?;

	let fresh_jwt = out.verify.jwt.clone();

	let input = sentc_crypto_light::user::change_password(old_pw, new_pw, &res, &res2)?;

	//now change the pw
	let url = base_url + "/api/v1/customer/password";

	let res = make_req(
		HttpMethod::PUT,
		url.as_str(),
		"",
		Some(input),
		Some(fresh_jwt.as_str()),
		None,
	)
	.await?;

	Ok(handle_general_server_response(res.as_str())?)
}
