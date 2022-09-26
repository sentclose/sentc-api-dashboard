import {Action, Module, Mutation, VuexModule} from "vuex-module-decorators";
import {CustomerLoginData} from "~/utils/types";

/**
 * @author Jörn Heinemann <joernheinemann@gmx.de>
 * @since 2022/09/26
 */

@Module({
	stateFactory: true
})
export default class Customer extends VuexModule
{
	private jwt = "";

	private refresh_token = "";

	private email = "";

	private validate_email = false;

	//time when the email was sent
	private email_send = 0;

	//0 = normal, other value = err
	private email_status = 0;

	private user_id = "";

	private device_id = "";

	//use a var because if anyone tries to access the public or private keys without init the store -> error
	//0 = can't login not login data
	//1 = is log in
	//2 = can try to log in with refresh token
	private isLoggedIn = 2;

	get getUserId() {
		return this.user_id;
	}

	get getJwt() {
		return this.jwt;
	}

	get loggedIn() {
		return this.isLoggedIn;
	}

	get getEmail() {
		return this.email;
	}

	@Mutation
	public setJwt(token: string)
	{
		this.jwt = token;
	}

	@Mutation
	public setLoginStatus(status: number)
	{
		this.isLoggedIn = status;
	}

	@Mutation
	public setData(data: CustomerLoginData)
	{
		this.jwt = data.jwt;
		this.user_id = data.user_id;
		this.device_id = data.device_id;
		this.validate_email = data.validate_email;
		this.email_status = data.email_status;
		//@ts-ignore
		this.email_send = data.email_send;
		this.email = data.email;
		this.refresh_token = data.refresh_token;
	}
}