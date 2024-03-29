#![no_std]
#![allow(deprecated)]

mod app;
mod customer;
mod group;
mod utils;

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use server_dashboard_common::app::{AppFileOptionsInput, AppGroupOption, AppOptions};
use server_dashboard_common::sdk_common::group::GroupUserListItem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CaptchaCreateOutput
{
	captcha_id: String,
	png: String,
}

impl From<server_dashboard_common::sdk_common::user::CaptchaCreateOutput> for CaptchaCreateOutput
{
	fn from(o: server_dashboard_common::sdk_common::user::CaptchaCreateOutput) -> Self
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

impl From<sentc_crypto_utils::jwt::Claims> for Claims
{
	fn from(claims: sentc_crypto_utils::jwt::Claims) -> Self
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
pub struct PrepareLoginOtpOutput
{
	master_key: String,
	auth_key: String,
}

impl From<customer::PrepareLoginOtpOutput> for PrepareLoginOtpOutput
{
	fn from(value: customer::PrepareLoginOtpOutput) -> Self
	{
		Self {
			master_key: value.master_key,
			auth_key: value.auth_key,
		}
	}
}

#[wasm_bindgen]
pub struct UserLoginOut
{
	user_data: Option<CustomerVerifyLoginOutput>,

	mfa: Option<PrepareLoginOtpOutput>,
}

impl From<customer::PreLoginOut> for UserLoginOut
{
	fn from(value: customer::PreLoginOut) -> Self
	{
		match value {
			customer::PreLoginOut::Direct(d) => {
				Self {
					mfa: None,
					user_data: Some(d.into()),
				}
			},
			customer::PreLoginOut::Otp(d) => {
				Self {
					user_data: None,
					mfa: Some(d.into()),
				}
			},
		}
	}
}

#[wasm_bindgen]
impl UserLoginOut
{
	pub fn get_email(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.email_data.email.clone())
	}

	pub fn get_validate_email(&self) -> Option<bool>
	{
		self.user_data.as_ref().map(|o| o.email_data.validate_email)
	}

	pub fn get_email_send(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.email_data.email_send.to_string())
	}

	pub fn get_email_status(&self) -> Option<i32>
	{
		self.user_data.as_ref().map(|o| o.email_data.email_status)
	}

	pub fn get_user_id(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.user_id.clone())
	}

	pub fn get_jwt(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.jwt.clone())
	}

	pub fn get_device_id(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.device_id.clone())
	}

	pub fn get_refresh_token(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.refresh_token.clone())
	}

	pub fn get_name(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.email_data.name.clone())
	}

	pub fn get_first_name(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.email_data.first_name.clone())
	}

	pub fn get_company(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.and_then(|o| o.email_data.company.clone())
	}

	pub fn get_mfa_master_key(&self) -> Option<String>
	{
		self.mfa.as_ref().map(|o| o.master_key.clone())
	}

	pub fn get_mfa_auth_key(&self) -> Option<String>
	{
		self.mfa.as_ref().map(|o| o.auth_key.clone())
	}
}

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

impl From<server_dashboard_common::customer::CustomerDataOutput> for CustomerEmailData
{
	fn from(data: server_dashboard_common::customer::CustomerDataOutput) -> Self
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
pub struct CustomerVerifyLoginOutput
{
	jwt: String,
	refresh_token: String,
	user_id: String,
	device_id: String,
	email_data: CustomerEmailData,
}

impl From<customer::CustomerVerifyLoginOutput> for CustomerVerifyLoginOutput
{
	fn from(data: customer::CustomerVerifyLoginOutput) -> Self
	{
		Self {
			jwt: data.jwt,
			refresh_token: data.refresh_token,
			user_id: data.user_id,
			device_id: data.device_id,
			email_data: data.email_data.into(),
		}
	}
}

#[wasm_bindgen]
impl CustomerVerifyLoginOutput
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
		self.user_id.clone()
	}

	pub fn get_jwt(&self) -> String
	{
		self.jwt.clone()
	}

	pub fn get_device_id(&self) -> String
	{
		self.device_id.clone()
	}

	pub fn get_refresh_token(&self) -> String
	{
		self.refresh_token.clone()
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
	let claims = sentc_crypto_utils::jwt::decode_jwt(jwt).map_err(Into::<String>::into)?;

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
	Ok(customer::refresh_jwt(base_url, old_jwt.as_str(), refresh_token).await?)
}

#[wasm_bindgen]
pub async fn get_fresh_jwt(
	base_url: String,
	user_identifier: String,
	password: String,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<String, JsValue>
{
	Ok(customer::get_fresh_jwt(base_url, &user_identifier, &password, mfa_token, mfa_recovery).await?)
}

#[wasm_bindgen]
pub async fn login(base_url: String, email: String, password: String) -> Result<UserLoginOut, JsValue>
{
	let out = customer::login(base_url, email.as_str(), password.as_str()).await?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn mfa_login(
	base_url: String,
	master_key_encryption: String,
	auth_key: String,
	user_identifier: String,
	token: String,
	recovery: bool,
) -> Result<CustomerVerifyLoginOutput, JsValue>
{
	let data = customer::mfa_login(
		base_url,
		&master_key_encryption,
		auth_key,
		user_identifier,
		token,
		recovery,
	)
	.await?;

	Ok(data.into())
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
pub async fn delete_customer(base_url: String, fresh_jwt: String) -> Result<(), JsValue>
{
	Ok(customer::delete_customer(base_url, &fresh_jwt).await?)
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
pub async fn change_password(
	base_url: String,
	email: String,
	old_pw: String,
	new_pw: String,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<(), JsValue>
{
	Ok(customer::change_password(
		base_url,
		email.as_str(),
		old_pw.as_str(),
		new_pw.as_str(),
		mfa_token,
		mfa_recovery,
	)
	.await?)
}

#[wasm_bindgen]
pub struct OtpRegisterUrl
{
	url: String,
	recover: Vec<String>,
}

#[wasm_bindgen]
impl OtpRegisterUrl
{
	pub fn get_url(&self) -> String
	{
		self.url.clone()
	}

	pub fn get_recover(&self) -> JsValue
	{
		JsValue::from_serde(&self.recover).unwrap()
	}
}

#[wasm_bindgen]
pub async fn register_otp(base_url: String, issuer: String, audience: String, fresh_jwt: String) -> Result<OtpRegisterUrl, JsValue>
{
	let (url, recover) = customer::register_otp(base_url, &issuer, &audience, &fresh_jwt).await?;

	Ok(OtpRegisterUrl {
		url,
		recover,
	})
}

#[wasm_bindgen]
pub async fn reset_otp(base_url: String, issuer: String, audience: String, fresh_jwt: String) -> Result<OtpRegisterUrl, JsValue>
{
	let (url, recover) = customer::reset_otp(base_url, &issuer, &audience, &fresh_jwt).await?;

	Ok(OtpRegisterUrl {
		url,
		recover,
	})
}

#[wasm_bindgen]
pub async fn get_otp_recover_keys(base_url: String, jwt: String) -> Result<JsValue, JsValue>
{
	let out = customer::get_otp_recover_keys(base_url, &jwt).await?;

	Ok(JsValue::from_serde(&out.keys).unwrap())
}

#[wasm_bindgen]
pub async fn disable_otp(base_url: String, fresh_jwt: String) -> Result<(), JsValue>
{
	Ok(customer::disable_otp(base_url, &fresh_jwt).await?)
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

impl From<server_dashboard_common::app::AppJwtRegisterOutput> for AppJwtRegisterOutput
{
	fn from(out: server_dashboard_common::app::AppJwtRegisterOutput) -> Self
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

impl From<server_dashboard_common::app::AppRegisterOutput> for AppRegisterOutput
{
	fn from(out: server_dashboard_common::app::AppRegisterOutput) -> Self
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

impl From<server_dashboard_common::app::AppTokenRenewOutput> for AppTokenRenewOutput
{
	fn from(out: server_dashboard_common::app::AppTokenRenewOutput) -> Self
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

impl From<server_dashboard_common::customer::CustomerAppList> for AppList
{
	fn from(d: server_dashboard_common::customer::CustomerAppList) -> Self
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

impl From<server_dashboard_common::app::AppDetails> for AppDetails
{
	fn from(d: server_dashboard_common::app::AppDetails) -> Self
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

impl From<server_dashboard_common::customer::CustomerGroupList> for CustomerGroupList
{
	fn from(value: server_dashboard_common::customer::CustomerGroupList) -> Self
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
	apps: Vec<server_dashboard_common::customer::CustomerAppList>,
}

impl From<server_dashboard_common::customer::CustomerGroupView> for CustomerGroupView
{
	fn from(value: server_dashboard_common::customer::CustomerGroupView) -> Self
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
	customer_data: Vec<server_dashboard_common::customer::CustomerList>,
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

impl From<server_dashboard_common::customer::CustomerGroupMemberFetch> for CustomerGroupMemberFetch
{
	fn from(value: server_dashboard_common::customer::CustomerGroupMemberFetch) -> Self
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
