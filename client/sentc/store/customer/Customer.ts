import {Action, Module, Mutation, VuexModule} from "vuex-module-decorators";
import {CustomerLoginData, USER_KEY_STORAGE_NAMES} from "~/utils/types";
import {decode_jwt, refresh_jwt} from "server_dashboard_wasm/server_dashboard_wasm_cjs";
import {Claims} from "server_dashboard_wasm/server_dashboard_wasm";
import {Storage} from "~/utils/FileStorage";

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
	public setEmail(email:string)
	{
		this.email = email;
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
		this.isLoggedIn = 1;
	}

	@Mutation
	public setLogout()
	{
		this.jwt = "";
		this.refresh_token = "";
		this.email = "";
		this.validate_email = false;
		this.email_send = 0;
		this.email_status = 0;
		this.user_id = "";
		this.device_id = "";
	}

	// eslint-disable-next-line require-await
	@Action({rawError: true})
	public async saveData(data: CustomerLoginData)
	{
		this.context.commit("setData", data);

		const storage = await Storage.getStore();

		await storage.set(USER_KEY_STORAGE_NAMES.userData, data);
	}

	@Action({rawError: true})
	public async logout()
	{
		this.context.commit("setLogout");

		const storage = await Storage.getStore();

		await storage.delete(USER_KEY_STORAGE_NAMES.userData);
	}

	@Action({rawError: true})
	public async getJwt()
	{
		const jwt_data: Claims = decode_jwt(this.jwt);
		const exp = jwt_data.get_exp();

		if (exp <= Date.now() / 1000 + 30) {
			const jwt = await refresh_jwt(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, this.jwt, this.refresh_token);

			this.context.commit("setJwt", jwt);

			const storage = await Storage.getStore();

			const data: CustomerLoginData = await storage.getItem(USER_KEY_STORAGE_NAMES.userData);

			data.jwt = jwt;

			await storage.set(USER_KEY_STORAGE_NAMES.userData, data);
		}

		return this.jwt;
	}
}