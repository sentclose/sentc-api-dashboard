import {Action, Module, Mutation, VuexModule} from "vuex-module-decorators";
import {AppData, AppDetails, AppFileOptions, AppJwtData, AppOptions, SentcError} from "~/utils/types";
import {get_all_apps, get_app, get_app_jwt_data} from "server_dashboard_wasm";

/**
 * @author Jörn Heinemann <joernheinemann@gmx.de>
 * @since 2022/09/27
 */

@Module({
	stateFactory: true
})
export default class App extends VuexModule
{
	private app_data: Map<string, AppData> = new Map<string, AppData>();

	private app_details: Map<string, AppDetails> = new Map<string, AppDetails>();

	private app_jwt_data: Map<string, AppJwtData[]> = new Map<string, AppJwtData[]>();

	private app_list: string[] = [];

	private app_list_end = false;

	get appList() {
		return this.app_list;
	}

	get appListEnd() {
		return this.app_list_end;
	}

	get app() {
		return (app_id: string) => {
			return this.app_data.get(app_id);
		};
	}

	get appDetails() {
		return (app_id: string) => {
			return this.app_details.get(app_id);
		};
	}

	get jwtData() {
		return (app_id: string) => {
			return this.app_jwt_data.get(app_id);
		};
	}

	@Mutation
	public setApp(data: AppData)
	{
		//push a new app to the front
		this.app_list.unshift(data.id);

		this.app_data.set(data.id, data);
	}

	@Mutation
	public setAppDetails(data: AppDetails)
	{
		this.app_details.set(data.id, data);
	}

	@Mutation
	public setAppIdentifier(data: {id: string, identifier: string})
	{
		const {id, identifier} = data;

		const app = this.app_details.get(id);

		if (app) {
			app.identifier = identifier;
		}

		//set it in app overview too

		const app_view = this.app_data.get(id);

		if (app_view) {
			app_view.identifier = identifier;
		}
	}

	@Mutation
	public setAppOptions(data: {id: string, options: AppOptions})
	{
		const app = this.app_details.get(data.id);

		if (!app) {
			return;
		}

		app.options = data.options;
	}

	@Mutation
	public setAppFileOptions(data: {id: string, options: AppFileOptions})
	{
		const app = this.app_details.get(data.id);

		if (!app) {
			return;
		}

		app.file_options = data.options;
	}

	@Mutation
	public setApps(data: AppData[])
	{
		if (data.length === 0) {
			this.app_list_end = true;
			return;
		}

		if (data.length < 20) {
			this.app_list_end = true;
		}

		for (let i = 0; i < data.length; i++) {
			const datum = data[i];

			this.app_list.push(datum.id);

			this.app_data.set(datum.id, datum);
		}
	}

	@Mutation
	public setAppJwtData(data: {jwt_data: AppJwtData[], id: string})
	{
		const {id, jwt_data} = data;

		this.app_jwt_data.set(id, jwt_data);
	}

	@Mutation
	public pushAppJwtData(data: {jwt_data: AppJwtData, id: string})
	{
		const {id, jwt_data} = data;

		if (!this.app_jwt_data.has(id)) {
			this.app_jwt_data.set(id, []);
		}

		this.app_jwt_data.get(id).unshift(jwt_data);
	}

	@Mutation
	public removeApp(id: string)
	{
		for (let i = 0; i < this.app_list.length; i++) {
			if (this.app_list[i] === id) {
				this.app_list.splice(i, 1);
			}
		}

		this.app_data.delete(id);
		this.app_jwt_data.delete(id);
		this.app_details.delete(id);
	}

	@Mutation
	public removeJwtData(data: {jwt_id: string, app_id: string})
	{
		const {jwt_id, app_id} = data;

		const app = this.app_jwt_data.get(app_id);

		if (!app) {
			return;
		}

		for (let i = 0; i < app.length; i++) {
			if (app[i].jwt_key_id === jwt_id) {
				app.splice(i, 1);
				return;
			}
		}
	}

	@Action({rawError: true})
	public async fetchApps()
	{
		if (this.app_list_end) {
			return;
		}

		const last = this.app_list[this.app_list.length - 1] ?? "none";
		const last_time = this.app_data.get(last)?.time.toString() ?? "0";

		let apps: AppData[] = [];

		try {
			const jwt = await this.context.dispatch("customer/Customer/getJwt", {}, {root: true});

			apps = await get_all_apps(process.env.NUXT_ENV_BASE_URL, jwt, last_time, last);
		} catch (e) {
			//no need to set error
		}

		this.context.commit("setApps", apps);
	}

	@Action({rawError: true})
	public async fetchJwtData(app_id: string)
	{
		try {
			const jwt = await this.context.dispatch("customer/Customer/getJwt", {}, {root: true});

			const jwt_data: AppJwtData[] = await get_app_jwt_data(process.env.NUXT_ENV_BASE_URL, jwt, app_id);

			this.context.commit("setAppJwtData", {jwt_data, id: app_id});
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.context.commit("event/ErrorEvent/setMsg", err.error_message, {root: true});
			} catch (e) {
				this.context.commit("event/ErrorEvent/setMsg", "An undefined error", {root: true});
			}
		}
	}

	@Action({rawError: true})
	public async fetchDetails(app_id: string)
	{
		try {
			const jwt = await this.context.dispatch("customer/Customer/getJwt", {}, {root: true});

			const data = await get_app(process.env.NUXT_ENV_BASE_URL, jwt, app_id);
			const details: AppDetails = {
				id: data.get_id(),
				identifier: data.get_identifier(),
				time: data.get_time() as unknown as number,
				file_options: data.get_file_options(),
				options: data.get_options()
			};

			this.context.commit("setAppDetails", details);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.context.commit("event/ErrorEvent/setMsg", err.error_message, {root: true});
			} catch (e) {
				this.context.commit("event/ErrorEvent/setMsg", "An undefined error", {root: true});
			}
		}
	}
}