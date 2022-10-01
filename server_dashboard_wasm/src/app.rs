use alloc::string::String;
use alloc::vec::Vec;

use sentc_crypto::util::public::{handle_general_server_response, handle_server_response};
use sentc_crypto_full::util::{make_req, HttpMethod};
use server_api_common::app::{AppFileOptionsInput, AppJwtData, AppOptions, AppRegisterInput, AppUpdateInput};
use server_api_common::customer::CustomerAppList;

use crate::utils;

pub async fn create(
	base_url: String,
	jwt: &str,
	identifier: Option<String>,
	options: AppOptions,
	file_options: AppFileOptionsInput,
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

pub async fn delete_jwt_keys(base_url: String, jwt: &str, app_id: &str, jwt_id: &str) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id + "/jwt/" + jwt_id;

	let res = make_req(HttpMethod::DELETE, url.as_str(), "", None, Some(jwt)).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn get_app_jwt_data(base_url: String, jwt: &str, app_id: &str) -> Result<Vec<AppJwtData>, String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id + "/jwt";

	let res = make_req(HttpMethod::GET, url.as_str(), "", None, Some(jwt)).await?;

	let out: Vec<AppJwtData> = handle_server_response(res.as_str())?;

	Ok(out)
}

pub async fn get_all_apps(base_url: String, jwt: &str, last_fetched_time: &str, last_id: &str) -> Result<Vec<CustomerAppList>, String>
{
	let url = base_url + "/api/v1/customer/apps/" + last_fetched_time + "/" + last_id;

	let res = make_req(HttpMethod::GET, url.as_str(), "", None, Some(jwt)).await?;

	let out: Vec<CustomerAppList> = handle_server_response(res.as_str())?;

	Ok(out)
}

pub async fn get_app(base_url: String, jwt: &str, app_id: &str) -> Result<server_api_common::app::AppDetails, String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id;

	let res = make_req(HttpMethod::GET, url.as_str(), "", None, Some(jwt)).await?;

	let out: server_api_common::app::AppDetails = handle_server_response(res.as_str())?;

	Ok(out)
}

pub async fn update_options(base_url: String, jwt: &str, app_id: &str, options: AppOptions) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id + "/options";

	let input = utils::to_string(&options)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt)).await?;

	handle_general_server_response(res.as_str())
}

pub async fn update_file_options(base_url: String, jwt: &str, app_id: &str, options: AppFileOptionsInput) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id + "/file_options";

	let input = utils::to_string(&options)?;

	let res = make_req(HttpMethod::PUT, url.as_str(), "", Some(input), Some(jwt)).await?;

	handle_general_server_response(res.as_str())
}

pub async fn delete_app(base_url: String, jwt: &str, app_id: &str) -> Result<(), String>
{
	let url = base_url + "/api/v1/customer/app/" + app_id;

	let res = make_req(HttpMethod::DELETE, url.as_str(), "", None, Some(jwt)).await?;

	handle_general_server_response(res.as_str())
}
