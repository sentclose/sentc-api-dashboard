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