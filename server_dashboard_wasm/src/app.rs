use alloc::string::String;

use sentc_crypto::util::public::{handle_general_server_response, handle_server_response};
use sentc_crypto_full::util::{make_req, HttpMethod};
use server_api_common::app::{AppFileOptions, AppOptions, AppRegisterInput, AppUpdateInput};

use crate::utils;

pub async fn create(
	base_url: String,
	jwt: &str,
	identifier: Option<String>,
	options: AppOptions,
	file_options: AppFileOptions,
) -> Result<server_api_common::app::AppRegisterOutput, String>
{
	let input = AppRegisterInput {
		identifier,
		options,
		file_options,
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/app";

	let res = make_req(HttpMethod::POST, url.as_str(), "", Some(input), Some(jwt)).await?;

	let out: server_api_common::app::AppRegisterOutput = handle_server_response(res.as_str())?;

	Ok(out)
}

pub async fn update(base_url: String, jwt: &str, app_id: &str, identifier: Option<String>) -> Result<(), String>
{
	let input = AppUpdateInput {
		identifier,
	};
	let input = utils::to_string(&input)?;

	let url = base_url + "/api/v1/customer/app/" + app_id;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt)).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn renew_token(base_url: String, jwt: &str, app_id: &str) -> Result<server_api_common::app::AppTokenRenewOutput, String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id + "/token_renew";

	let res = make_req(HttpMethod::PATCH, url.as_str(), "", None, Some(jwt)).await?;

	let out: server_api_common::app::AppTokenRenewOutput = handle_server_response(res.as_str())?;

	Ok(out)
}

pub async fn new_jwt_keys(base_url: String, jwt: &str, app_id: &str) -> Result<server_api_common::app::AppJwtRegisterOutput, String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id + "/new_jwt_keys";

	let res = make_req(HttpMethod::PATCH, url.as_str(), "", None, Some(jwt)).await?;

	let out: server_api_common::app::AppJwtRegisterOutput = handle_server_response(res.as_str())?;

	Ok(out)
}
