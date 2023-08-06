export interface SentcError {
	status: string,
	error_message: string
}

export interface CustomerLoginData
{
	email: string,
	validate_email: boolean,
	email_send: string,	//the time stamp
	email_status: number, //error code
	user_id: string,
	jwt: string,
	refresh_token: string,
	device_id: string,
	name: string,
	first_name: string,
	company?: string
}

export const enum USER_KEY_STORAGE_NAMES
{
	userData = "user_data",
	actualUser = "actual_user",

	userPublicData = "user_public_data",
	userPublicKey = "user_public_key",
	userVerifyKey = "user_verify_key",

	groupData = "group_data"
}

export interface AppData
{
	id: string,
	identifier: string,
	time: number,
	group_name?: string,
}

export interface AppDetails
{
	id: string,
	identifier: string,
	time: number,
	options: AppOptions,
	file_options: AppFileOptions,
	group_options: AppGroupOptions
}

export interface AppJwtData {
	jwt_key_id: string,
	jwt_alg: string, //should be ES384 for now
	time: number,
	sign_key: string,
	verify_key: string,
}

export interface AppFileOptions {
	file_storage: number,
	storage_url?: string,
	auth_token?: string
}

export interface AppGroupOptions {
	max_key_rotation_month: number,
	min_rank_key_rotation: number
}

export interface AppOptions
{
	group_create: number,
	group_get: number,

	group_user_keys: number,
	group_user_update_check: number,

	group_invite: number,
	group_auto_invite: number,
	group_reject_invite: number,
	group_accept_invite: number,

	group_join_req: number,
	group_accept_join_req: number,
	group_reject_join_req: number,

	group_key_rotation: number,

	group_user_delete: number,
	group_delete: number,

	group_leave: number,
	group_change_rank: number,

	user_exists: number,
	user_register: number,
	user_delete: number,
	user_update: number,
	user_change_password: number,
	user_reset_password: number,
	user_prepare_login: number,
	user_done_login: number,
	user_public_data: number,
	user_jwt_refresh: number,

	key_register: number,
	key_get: number,

	group_list: number,

	file_register: number,
	file_part_upload: number,
	file_get: number,
	file_part_download: number,
	file_delete: number,

	user_device_register: number,
	user_device_delete: number,
	user_device_list: number,

	group_invite_stop: number,

	user_key_update: number,

	content: number,

	content_small: number,
	content_med: number,
	content_large: number,
	content_x_large: number,

	user_register_otp: number,
	user_reset_otp: number,
	user_disable_otp: number,
	user_get_otp_recovery_keys: number,
}

//______________________________________________________________________________________________________________________

export interface GroupData
{
	id: string,
	time: string,
	rank: number,
	group_name?: string,
	des?: string,
}

export interface GroupUserListItem
{
	user_id: string,
	rank: number,
	joined_time: number,
	user_type: number,
}

export interface CustomerList
{
	id: string,
	first_name: string,
	name: string,
	email: string,
}

export interface CustomerGroupMemberListItem
{
	user_id: string,
	rank: number,
	joined_time: number,
	first_name: string,
	name: string,
	email: string,
}