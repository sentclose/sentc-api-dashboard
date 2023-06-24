#![no_std]

mod app;
mod customer;
mod group;
mod utils;

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use server_api_common::app::{AppFileOptionsInput, AppGroupOption, AppOptions};
use server_api_common::customer::CustomerList;
use server_api_common::sdk_common::group::GroupUserListItem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CaptchaCreateOutput
{
	captcha_id: String,
	png: String,
}

impl From<server_api_common::sdk_common::user::CaptchaCreateOutput> for CaptchaCreateOutput
{
	fn from(o: server_api_common::sdk_common::user::CaptchaCreateOutput) -> Self
	{
		Self {
			captcha_id: o.captcha_id,
			png: o.png,
		}
	}
}

#[wasm_bindgen]
impl CaptchaCreateOutput
{
	pub fn get_captcha_id(&self) -> String
	{
		self.captcha_id.clone()
	}

	pub fn get_png(&self) -> String
	{
		self.png.clone()
	}
}

#[wasm_bindgen]
pub struct Claims
{
	aud: String,
	sub: String, //the app id
	exp: usize,
	iat: usize,
	fresh: bool, //was this token from refresh jwt or from login
}

#[wasm_bindgen]
impl Claims
{
	pub fn get_aud(&self) -> String
	{
		self.aud.clone()
	}

	pub fn get_sub(&self) -> String
	{
		self.sub.clone()
	}

	pub fn get_exp(&self) -> usize
	{
		self.exp
	}

	pub fn get_iat(&self) -> usize
	{
		self.iat
	}

	pub fn get_fresh(&self) -> bool
	{
		self.fresh
	}
}

impl From<sentc_crypto_full::jwt::Claims> for Claims
{
	fn from(claims: sentc_crypto_full::jwt::Claims) -> Self
	{
		Self {
			aud: claims.aud,
			sub: claims.sub,
			exp: claims.exp,
			iat: claims.iat,
			fresh: claims.fresh,
		}
	}
}

//__________________________________________________________________________________________________

#[wasm_bindgen]
pub struct CustomerEmailData
{
	validate_email: bool,
	email: String,
	email_send: u128,
	email_status: i32,
	name: String,
	first_name: String,
	company: Option<String>,
}

impl From<server_api_common::customer::CustomerDataOutput> for CustomerEmailData
{
	fn from(data: server_api_common::customer::CustomerDataOutput) -> Self
	{
		Self {
			validate_email: data.validate_email,
			email: data.email,
			email_send: data.email_send,
			email_status: data.email_status,
			name: data.name,
			first_name: data.first_name,
			company: data.company,
		}
	}
}

#[wasm_bindgen]
pub struct DoneLoginLightOutput
{
	user_id: String,
	jwt: String,
	device_id: String,
	refresh_token: String,
}

impl From<server_api_common::sdk_common::user::DoneLoginLightOutput> for DoneLoginLightOutput
{
	fn from(key: server_api_common::sdk_common::user::DoneLoginLightOutput) -> Self
	{
		Self {
			user_id: key.user_id,
			jwt: key.jwt,
			device_id: key.device_id,
			refresh_token: key.refresh_token,
		}
	}
}

#[wasm_bindgen]
pub struct CustomerDoneLoginOutput
{
	user_keys: DoneLoginLightOutput,
	email_data: CustomerEmailData,
}

impl From<server_api_common::customer::CustomerDoneLoginOutput> for CustomerDoneLoginOutput
{
	fn from(data: server_api_common::customer::CustomerDoneLoginOutput) -> Self
	{
		Self {
			user_keys: data.user_keys.into(),
			email_data: data.email_data.into(),
		}
	}
}

#[wasm_bindgen]
impl CustomerDoneLoginOutput
{
	pub fn get_email(&self) -> String
	{
		self.email_data.email.clone()
	}

	pub fn get_validate_email(&self) -> bool
	{
		self.email_data.validate_email
	}

	pub fn get_email_send(&self) -> String
	{
		self.email_data.email_send.to_string()
	}

	pub fn get_email_status(&self) -> i32
	{
		self.email_data.email_status
	}

	pub fn get_user_id(&self) -> String
	{
		self.user_keys.user_id.clone()
	}

	pub fn get_jwt(&self) -> String
	{
		self.user_keys.jwt.clone()
	}

	pub fn get_device_id(&self) -> String
	{
		self.user_keys.device_id.clone()
	}

	pub fn get_refresh_token(&self) -> String
	{
		self.user_keys.refresh_token.clone()
	}

	pub fn get_name(&self) -> String
	{
		self.email_data.name.clone()
	}

	pub fn get_first_name(&self) -> String
	{
		self.email_data.first_name.clone()
	}

	pub fn get_company(&self) -> Option<String>
	{
		self.email_data.company.clone()
	}
}

#[wasm_bindgen]
pub fn decode_jwt(jwt: &str) -> Result<Claims, JsValue>
{
	let claims = sentc_crypto_full::jwt::decode_jwt(jwt)?;

	Ok(claims.into())
}

#[wasm_bindgen]
pub async fn captcha_req(base_url: String) -> Result<CaptchaCreateOutput, JsValue>
{
	let out = customer::captcha_req(base_url).await?;

	Ok(out.into())
}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub async fn register(
	base_url: String,
	email: String,
	password: String,
	name: String,
	first_name: String,
	company: String,
	captcha_solution: String,
	captcha_id: String,
) -> Result<String, JsValue>
{
	let company = match company.as_str() {
		"" => None,
		_ => Some(company),
	};

	let out = customer::register(
		base_url,
		email,
		password.as_str(),
		name,
		first_name,
		company,
		captcha_solution,
		captcha_id,
	)
	.await?;

	Ok(out)
}

#[wasm_bindgen]
pub async fn done_register(base_url: String, jwt: String, token: String) -> Result<(), JsValue>
{
	Ok(customer::done_register(base_url, jwt.as_str(), token).await?)
}

#[wasm_bindgen]
pub async fn refresh_jwt(base_url: String, old_jwt: String, refresh_token: String) -> Result<String, JsValue>
{
	Ok(customer::refresh_jwt(base_url, old_jwt.as_str(), refresh_token.as_str()).await?)
}

#[wasm_bindgen]
pub async fn login(base_url: String, email: String, password: String) -> Result<CustomerDoneLoginOutput, JsValue>
{
	let out = customer::login(base_url, email.as_str(), password.as_str()).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn update(base_url: String, jwt: String, new_email: String) -> Result<(), JsValue>
{
	Ok(customer::update(base_url, jwt.as_str(), new_email).await?)
}

#[wasm_bindgen]
pub async fn update_data(base_url: String, jwt: String, name: String, first_name: String, company: String) -> Result<(), JsValue>
{
	let company = match company.as_str() {
		"" => None,
		_ => Some(company),
	};

	Ok(customer::update_data(base_url, jwt.as_str(), name, first_name, company).await?)
}

#[wasm_bindgen]
pub async fn delete_customer(base_url: String, email: String, pw: String) -> Result<(), JsValue>
{
	Ok(customer::delete_customer(base_url, email.as_str(), pw.as_str()).await?)
}

#[wasm_bindgen]
pub async fn prepare_reset_password(base_url: String, email: String, captcha_solution: String, captcha_id: String) -> Result<(), JsValue>
{
	Ok(customer::prepare_reset_password(base_url, email, captcha_solution, captcha_id).await?)
}

#[wasm_bindgen]
pub async fn done_reset_password(base_url: String, token: String, email: String, new_pw: String) -> Result<(), JsValue>
{
	Ok(customer::done_reset_password(base_url, token, email.as_str(), new_pw.as_str()).await?)
}

#[wasm_bindgen]
pub async fn change_password(base_url: String, email: String, old_pw: String, new_pw: String) -> Result<(), JsValue>
{
	Ok(customer::change_password(base_url, email.as_str(), old_pw.as_str(), new_pw.as_str()).await?)
}

//__________________________________________________________________________________________________

#[wasm_bindgen]
pub fn app_options_default() -> JsValue
{
	let out = AppOptions::default();

	JsValue::from_serde(&out).unwrap()
}

#[wasm_bindgen]
pub fn app_options_lax() -> JsValue
{
	let out = AppOptions::default_lax();

	JsValue::from_serde(&out).unwrap()
}

#[wasm_bindgen]
pub struct AppJwtRegisterOutput
{
	app_id: String,
	jwt_id: String,
	jwt_verify_key: String,
	jwt_sign_key: String,
	jwt_alg: String,
}

impl From<server_api_common::app::AppJwtRegisterOutput> for AppJwtRegisterOutput
{
	fn from(out: server_api_common::app::AppJwtRegisterOutput) -> Self
	{
		Self {
			app_id: out.app_id,
			jwt_id: out.jwt_id,
			jwt_verify_key: out.jwt_verify_key,
			jwt_sign_key: out.jwt_sign_key,
			jwt_alg: out.jwt_alg,
		}
	}
}

#[wasm_bindgen]
impl AppJwtRegisterOutput
{
	pub fn get_app_id(&self) -> String
	{
		self.app_id.clone()
	}

	pub fn get_jwt_id(&self) -> String
	{
		self.jwt_id.clone()
	}

	pub fn get_jwt_verify_key(&self) -> String
	{
		self.jwt_verify_key.clone()
	}

	pub fn get_jwt_sign_key(&self) -> String
	{
		self.jwt_sign_key.clone()
	}

	pub fn get_jwt_alg(&self) -> String
	{
		self.jwt_alg.clone()
	}
}

#[wasm_bindgen]
pub struct AppRegisterOutput
{
	app_id: String,
	secret_token: String,
	public_token: String,
	jwt_data: AppJwtRegisterOutput,
}

#[wasm_bindgen]
impl AppRegisterOutput
{
	pub fn get_app_id(&self) -> String
	{
		self.app_id.clone()
	}

	pub fn get_secret_token(&self) -> String
	{
		self.secret_token.clone()
	}

	pub fn get_public_token(&self) -> String
	{
		self.public_token.clone()
	}

	pub fn get_jwt_id(&self) -> String
	{
		self.jwt_data.jwt_id.clone()
	}

	pub fn get_jwt_verify_key(&self) -> String
	{
		self.jwt_data.jwt_verify_key.clone()
	}

	pub fn get_jwt_sign_key(&self) -> String
	{
		self.jwt_data.jwt_sign_key.clone()
	}

	pub fn get_jwt_alg(&self) -> String
	{
		self.jwt_data.jwt_alg.clone()
	}
}

impl From<server_api_common::app::AppRegisterOutput> for AppRegisterOutput
{
	fn from(out: server_api_common::app::AppRegisterOutput) -> Self
	{
		Self {
			app_id: out.app_id,
			secret_token: out.secret_token,
			public_token: out.public_token,
			jwt_data: out.jwt_data.into(),
		}
	}
}

#[wasm_bindgen]
pub struct AppTokenRenewOutput
{
	secret_token: String,
	public_token: String,
}

impl From<server_api_common::app::AppTokenRenewOutput> for AppTokenRenewOutput
{
	fn from(out: server_api_common::app::AppTokenRenewOutput) -> Self
	{
		Self {
			secret_token: out.secret_token,
			public_token: out.public_token,
		}
	}
}

#[wasm_bindgen]
impl AppTokenRenewOutput
{
	pub fn get_secret_token(&self) -> String
	{
		self.secret_token.clone()
	}

	pub fn get_public_token(&self) -> String
	{
		self.public_token.clone()
	}
}

#[wasm_bindgen]
pub struct AppList
{
	id: String,
	identifier: String,
	time: u128,
}

impl From<server_api_common::customer::CustomerAppList> for AppList
{
	fn from(d: server_api_common::customer::CustomerAppList) -> Self
	{
		Self {
			id: d.id,
			identifier: d.identifier,
			time: d.time,
		}
	}
}

#[wasm_bindgen]
pub struct AppDetails
{
	options: AppOptions,
	file_options: AppFileOptionsInput,
	group_options: AppGroupOption,
	details: AppList,
}

impl From<server_api_common::app::AppDetails> for AppDetails
{
	fn from(d: server_api_common::app::AppDetails) -> Self
	{
		Self {
			options: d.options,
			file_options: d.file_options,
			group_options: d.group_options,
			details: d.details.into(),
		}
	}
}

#[wasm_bindgen]
impl AppDetails
{
	pub fn get_id(&self) -> String
	{
		self.details.id.clone()
	}

	pub fn get_identifier(&self) -> String
	{
		self.details.identifier.clone()
	}

	pub fn get_time(&self) -> String
	{
		self.details.time.to_string()
	}

	pub fn get_options(&self) -> JsValue
	{
		JsValue::from_serde(&self.options).unwrap()
	}

	pub fn get_file_options(&self) -> JsValue
	{
		JsValue::from_serde(&self.file_options).unwrap()
	}

	pub fn get_group_options(&self) -> JsValue
	{
		JsValue::from_serde(&self.group_options).unwrap()
	}
}

#[wasm_bindgen]
pub async fn app_create_app(
	base_url: String,
	jwt: String,
	identifier: String,
	options: JsValue,
	file_options: JsValue,
	group_options: JsValue,
	group_id: Option<String>,
) -> Result<AppRegisterOutput, JsValue>
{
	let identifier = match identifier.as_str() {
		"" => None,
		_ => Some(identifier),
	};

	let options: AppOptions = options.into_serde().unwrap();
	let file_options: AppFileOptionsInput = file_options.into_serde().unwrap();
	let group_options: AppGroupOption = group_options.into_serde().unwrap();

	let out = app::create(
		base_url,
		jwt.as_str(),
		identifier,
		options,
		file_options,
		group_options,
		group_id,
	)
	.await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn app_update(base_url: String, jwt: String, app_id: String, identifier: String) -> Result<(), JsValue>
{
	let identifier = match identifier.as_str() {
		"" => None,
		_ => Some(identifier),
	};

	Ok(app::update(base_url, jwt.as_str(), app_id.as_str(), identifier).await?)
}

#[wasm_bindgen]
pub async fn renew_token(base_url: String, jwt: String, app_id: String) -> Result<AppTokenRenewOutput, JsValue>
{
	let out = app::renew_token(base_url, jwt.as_str(), app_id.as_str()).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn new_jwt_keys(base_url: String, jwt: String, app_id: String) -> Result<AppJwtRegisterOutput, JsValue>
{
	let out = app::new_jwt_keys(base_url, jwt.as_str(), app_id.as_str()).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn delete_jwt_keys(base_url: String, jwt: String, app_id: String, jwt_id: String) -> Result<(), JsValue>
{
	Ok(app::delete_jwt_keys(base_url, jwt.as_str(), app_id.as_str(), jwt_id.as_str()).await?)
}

#[wasm_bindgen]
pub async fn get_app_jwt_data(base_url: String, jwt: String, app_id: String) -> Result<JsValue, JsValue>
{
	let out = app::get_app_jwt_data(base_url, jwt.as_str(), app_id.as_str()).await?;

	Ok(JsValue::from_serde(&out).unwrap())
}

#[wasm_bindgen]
pub async fn get_all_apps(base_url: String, jwt: String, last_fetched_time: String, last_id: String) -> Result<JsValue, JsValue>
{
	let out = app::get_all_apps(base_url, jwt.as_str(), last_fetched_time.as_str(), last_id.as_str()).await?;

	Ok(JsValue::from_serde(&out).unwrap())
}

#[wasm_bindgen]
pub async fn get_all_apps_in_group(
	base_url: String,
	jwt: String,
	group_id: String,
	last_fetched_time: String,
	last_id: String,
) -> Result<JsValue, JsValue>
{
	let out = app::get_all_apps_in_group(
		base_url,
		jwt.as_str(),
		&group_id,
		last_fetched_time.as_str(),
		last_id.as_str(),
	)
	.await?;

	Ok(JsValue::from_serde(&out).unwrap())
}

#[wasm_bindgen]
pub async fn get_app(base_url: String, jwt: String, app_id: String) -> Result<AppDetails, JsValue>
{
	let out = app::get_app(base_url, jwt.as_str(), app_id.as_str()).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn app_update_options(base_url: String, jwt: String, app_id: String, options: JsValue) -> Result<(), JsValue>
{
	let options: AppOptions = options.into_serde().unwrap();

	Ok(app::update_options(base_url, jwt.as_str(), app_id.as_str(), options).await?)
}

#[wasm_bindgen]
pub async fn app_update_file_options(base_url: String, jwt: String, app_id: String, file_options: JsValue) -> Result<(), JsValue>
{
	let options: AppFileOptionsInput = file_options.into_serde().unwrap();

	Ok(app::update_file_options(base_url, jwt.as_str(), app_id.as_str(), options).await?)
}

#[wasm_bindgen]
pub async fn app_update_group_options(base_url: String, jwt: String, app_id: String, group_options: JsValue) -> Result<(), JsValue>
{
	let options: AppGroupOption = group_options.into_serde().unwrap();

	Ok(app::update_group_options(base_url, &jwt, &app_id, options).await?)
}

#[wasm_bindgen]
pub async fn app_delete(base_url: String, jwt: String, app_id: String) -> Result<(), JsValue>
{
	Ok(app::delete_app(base_url, jwt.as_str(), app_id.as_str()).await?)
}

#[wasm_bindgen]
pub async fn app_reset(base_url: String, jwt: String, app_id: String) -> Result<(), JsValue>
{
	Ok(app::reset_app(base_url, jwt.as_str(), app_id.as_str()).await?)
}

//__________________________________________________________________________________________________

#[wasm_bindgen]
pub struct CustomerGroupList
{
	id: String,
	time: String,
	rank: i32,
	group_name: Option<String>,
	des: Option<String>,
}

impl From<server_api_common::customer::CustomerGroupList> for CustomerGroupList
{
	fn from(value: server_api_common::customer::CustomerGroupList) -> Self
	{
		Self {
			id: value.id,
			time: value.time.to_string(),
			rank: value.rank,
			group_name: value.group_name,
			des: value.des,
		}
	}
}

#[wasm_bindgen]
pub struct CustomerGroupView
{
	data: CustomerGroupList,
	apps: Vec<server_api_common::customer::CustomerAppList>,
}

impl From<server_api_common::customer::CustomerGroupView> for CustomerGroupView
{
	fn from(value: server_api_common::customer::CustomerGroupView) -> Self
	{
		Self {
			data: value.data.into(),
			apps: value.apps,
		}
	}
}

#[wasm_bindgen]
impl CustomerGroupView
{
	pub fn get_id(&self) -> String
	{
		self.data.id.clone()
	}

	pub fn get_name(&self) -> Option<String>
	{
		self.data.group_name.clone()
	}

	pub fn get_time(&self) -> String
	{
		self.data.time.clone()
	}

	pub fn get_apps(&self) -> JsValue
	{
		JsValue::from_serde(&self.apps).unwrap()
	}

	pub fn get_des(&self) -> Option<String>
	{
		self.data.des.clone()
	}

	pub fn get_rank(&self) -> i32
	{
		self.data.rank
	}
}

#[wasm_bindgen]
pub async fn create_group(base_url: String, jwt: String, name: Option<String>, des: Option<String>) -> Result<String, JsValue>
{
	Ok(group::create_group(base_url, &jwt, name, des).await?)
}

#[wasm_bindgen]
pub async fn get_all_groups(base_url: String, jwt: String, last_fetched_time: String, last_id: String) -> Result<JsValue, JsValue>
{
	let out = group::get_all_groups(base_url, &jwt, &last_fetched_time, &last_id).await?;

	Ok(JsValue::from_serde(&out).unwrap())
}

#[wasm_bindgen]
pub async fn get_group(base_url: String, jwt: String, group_id: String) -> Result<CustomerGroupView, JsValue>
{
	let out = group::get_group(base_url, &jwt, &group_id).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn invite_member(base_url: String, jwt: String, group_id: String, user_id: String, rank: Option<i32>) -> Result<(), JsValue>
{
	Ok(group::invite_member(base_url, &jwt, &group_id, &user_id, rank).await?)
}

#[wasm_bindgen]
pub struct CustomerGroupMemberFetch
{
	group_member: Vec<GroupUserListItem>,
	customer_data: Vec<CustomerList>,
}

#[wasm_bindgen]
impl CustomerGroupMemberFetch
{
	pub fn get_member(&self) -> JsValue
	{
		JsValue::from_serde(&self.group_member).unwrap()
	}

	pub fn get_customer(&self) -> JsValue
	{
		JsValue::from_serde(&self.customer_data).unwrap()
	}
}

impl From<server_api_common::customer::CustomerGroupMemberFetch> for CustomerGroupMemberFetch
{
	fn from(value: server_api_common::customer::CustomerGroupMemberFetch) -> Self
	{
		Self {
			group_member: value.group_member,
			customer_data: value.customer_data,
		}
	}
}

#[wasm_bindgen]
pub async fn get_member_list(
	base_url: String,
	jwt: String,
	group_id: String,
	last_fetched_time: String,
	last_id: String,
) -> Result<CustomerGroupMemberFetch, JsValue>
{
	let out = group::get_member_list(base_url, &jwt, &group_id, &last_fetched_time, &last_id).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn update_user_rank(base_url: String, jwt: String, group_id: String, user_id: String, new_rank: i32) -> Result<(), JsValue>
{
	Ok(group::update_user_rank(base_url, &jwt, &group_id, user_id, new_rank).await?)
}

#[wasm_bindgen]
pub async fn kick_user(base_url: String, jwt: String, group_id: String, user_id: String) -> Result<(), JsValue>
{
	Ok(group::kick_user(base_url, &jwt, &group_id, &user_id).await?)
}

#[wasm_bindgen]
pub async fn update_group(base_url: String, jwt: String, group_id: String, name: Option<String>, des: Option<String>) -> Result<(), JsValue>
{
	Ok(group::update_group(base_url, &jwt, &group_id, name, des).await?)
}

#[wasm_bindgen]
pub async fn delete_group(base_url: String, jwt: String, group_id: String) -> Result<(), JsValue>
{
	Ok(group::delete_group(base_url, &jwt, &group_id).await?)
}
