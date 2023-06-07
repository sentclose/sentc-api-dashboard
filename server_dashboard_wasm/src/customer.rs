use alloc::string::{String, ToString};
use alloc::vec;

use sentc_crypto::util::public::{handle_general_server_response, handle_server_response};
use sentc_crypto::SdkError;
use sentc_crypto_full::util::{make_req, non_auth_req, HttpMethod};
use server_api_common::customer::{CustomerData, CustomerDoneRegistrationInput, CustomerRegisterData, CustomerRegisterOutput, CustomerUpdateInput};
use server_api_common::sdk_common::user::{
	CaptchaCreateOutput,
	CaptchaInput,
	ChangePasswordData,
	DoneLoginLightServerOutput,
	DoneLoginServerInput,
	DoneLoginServerKeysOutput,
	DoneLoginServerOutput,
	PrepareLoginSaltServerOutput,
	RegisterData,
};
use server_api_common::sdk_common::ServerOutput;

use crate::utils;

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
	let register_data = sentc_crypto::user::register(email.as_str(), password)?;
	let register_data = RegisterData::from_string(register_data.as_str()).map_err(SdkError::JsonParseFailed)?;

	let input = CustomerRegisterData {
		customer_data: CustomerData {
			name,
			first_name,
			company,
		},
		email,
		register_data: register_data.device,
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

pub async fn refresh_jwt(base_url: String, old_jwt: &str, refresh_token: &str) -> Result<String, String>
{
	let url = base_url + "/api/v1/customer/refresh";

	let input = sentc_crypto::user::prepare_refresh_jwt(refresh_token)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(old_jwt), None).await?;

	let server_output: DoneLoginLightServerOutput = handle_server_response(res.as_str())?;

	Ok(server_output.jwt)
}

pub async fn login(base_url: String, email: &str, password: &str) -> Result<server_api_common::customer::CustomerDoneLoginOutput, String>
{
	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let prep_server_input = sentc_crypto::user::prepare_login_start(email)?;

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (auth_key, _derived_master_key) = sentc_crypto::user::prepare_login(email, password, res.as_str())?;

	let url = base_url + "/api/v1/customer/done_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(auth_key)).await?;

	let out: server_api_common::customer::CustomerDoneLoginOutput = handle_server_response(res.as_str())?;

	Ok(out)
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
	let prep_server_input = sentc_crypto::user::prepare_login_start(email)?;

	//get a fresh jwt
	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (auth_key, _derived_master_key) = sentc_crypto::user::prepare_login(email, pw, res.as_str())?;

	let url = base_url.clone() + "/api/v1/customer/done_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(auth_key)).await?;

	let out: server_api_common::customer::CustomerDoneLoginOutput = handle_server_response(res.as_str())?;

	let fresh_jwt = out.user_keys.jwt;

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

	//make a fake register and login to get fake decrypted private keys, then do the pw reset like normal user
	//use a rand pw to generate the fake keys
	let (prepare_login_user_data, done_login_user_data) = get_fake_login_data("abc")?;
	//use the same fake pw here
	let (_auth_key, derived_master_key) = sentc_crypto::user::prepare_login(email, "abc", prepare_login_user_data.as_str())?;
	let user_key_data = sentc_crypto::user::done_login(&derived_master_key, done_login_user_data.as_str())?;

	let pw_reset_out = sentc_crypto::user::reset_password(
		new_pw,
		&user_key_data.device_keys.private_key,
		&user_key_data.device_keys.sign_key,
	)?;
	let reset_password_data =
		server_api_common::sdk_common::user::ResetPasswordData::from_string(pw_reset_out.as_str()).map_err(SdkError::JsonParseFailed)?;

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
	let prep_server_input = sentc_crypto::user::prepare_login_start(email)?;

	let url = base_url.clone() + "/api/v1/customer/prepare_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(prep_server_input)).await?;

	let (auth_key, _derived_master_key) = sentc_crypto::user::prepare_login(email, old_pw, res.as_str())?;

	//get the fake data to change the pw in the client. we don't need the user keys because customer don't got any keys
	let pw_change_data = get_fake_pw_change_data(auth_key.as_str(), old_pw, new_pw)?;

	let url = base_url.clone() + "/api/v1/customer/done_login";

	let res = non_auth_req(HttpMethod::POST, url.as_str(), "", Some(auth_key)).await?;

	let out: server_api_common::customer::CustomerDoneLoginOutput = handle_server_response(res.as_str())?;

	let fresh_jwt = out.user_keys.jwt;

	//now change the pw
	let url = base_url + "/api/v1/customer/password";

	let res = make_req(
		HttpMethod::PUT,
		url.as_str(),
		"",
		Some(pw_change_data),
		Some(fresh_jwt.as_str()),
		None,
	)
	.await?;

	Ok(handle_general_server_response(res.as_str())?)
}

//__________________________________________________________________________________________________

fn get_fake_pw_change_data(prepare_login_auth_key_input: &str, old_pw: &str, new_pw: &str) -> Result<String, String>
{
	let (prepare_login_user_data, done_login_user_data) = get_fake_login_data(old_pw)?;

	let pw_change_data = sentc_crypto::user::change_password(
		old_pw,
		new_pw,
		prepare_login_user_data.as_str(),
		done_login_user_data.as_str(),
	)?;

	let mut pw_change_data = ChangePasswordData::from_string(pw_change_data.as_str()).map_err(SdkError::JsonParseFailed)?;
	let auth_key = DoneLoginServerInput::from_string(prepare_login_auth_key_input).map_err(SdkError::JsonParseFailed)?;

	pw_change_data.old_auth_key = auth_key.auth_key;
	let pw_change_data = pw_change_data
		.to_string()
		.map_err(|_e| SdkError::JsonToStringFailed)?;

	Ok(pw_change_data)
}

fn get_fake_login_data(old_pw: &str) -> Result<(String, String), String>
{
	//use a fake master key to change the password,
	// just register the user again with fake data but with the old password to decrypt the fake data!
	let fake_key_data = sentc_crypto::user::register("abc", old_pw)?;
	let fake_key_data = RegisterData::from_string(fake_key_data.as_str()).map_err(SdkError::JsonParseFailed)?;

	//do the server prepare login again to get the salt (we need a salt to this fake register data)
	let salt_string = sentc_crypto::util::server::generate_salt_from_base64_to_string(
		fake_key_data.device.derived.client_random_value.as_str(),
		fake_key_data.device.derived.derived_alg.as_str(),
		"",
	)?;

	let prepare_login_user_data = PrepareLoginSaltServerOutput {
		salt_string,
		derived_encryption_key_alg: fake_key_data.device.derived.derived_alg,
	};

	let device_keys = DoneLoginServerKeysOutput {
		encrypted_master_key: fake_key_data.device.master_key.encrypted_master_key,
		encrypted_private_key: fake_key_data.device.derived.encrypted_private_key,
		public_key_string: fake_key_data.device.derived.public_key,
		keypair_encrypt_alg: fake_key_data.device.derived.keypair_encrypt_alg,
		encrypted_sign_key: fake_key_data.device.derived.encrypted_sign_key,
		verify_key_string: fake_key_data.device.derived.verify_key,
		keypair_sign_alg: fake_key_data.device.derived.keypair_sign_alg,
		keypair_encrypt_id: "abc".to_string(),
		keypair_sign_id: "abc".to_string(),
		user_id: "abc".to_string(),
		device_id: "1234".to_string(),
		user_group_id: "1234".to_string(),
	};

	let done_login_user_data = DoneLoginServerOutput {
		device_keys,
		jwt: "abc".to_string(),
		refresh_token: "abc".to_string(),
		user_keys: vec![],
		hmac_keys: vec![],
	};

	let prepare_login_user_data = ServerOutput {
		status: true,
		err_msg: None,
		err_code: None,
		result: Some(prepare_login_user_data),
	};
	let prepare_login_user_data = prepare_login_user_data
		.to_string()
		.map_err(|_e| SdkError::JsonToStringFailed)?;

	let done_login_user_data = ServerOutput {
		status: true,
		err_msg: None,
		err_code: None,
		result: Some(done_login_user_data),
	};
	let done_login_user_data = done_login_user_data
		.to_string()
		.map_err(|_e| SdkError::JsonToStringFailed)?;

	Ok((prepare_login_user_data, done_login_user_data))
}
