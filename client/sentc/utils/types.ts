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
	device_id: string
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