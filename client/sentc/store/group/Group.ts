/**
 * @author Jörn Heinemann <joernheinemann@gmx.de>
 * @since 2023/04/30
 */
import {Action, Module, Mutation, VuexModule} from "vuex-module-decorators";
import {AppData, GroupData} from "~/utils/types";
import {get_all_groups, get_group} from "server_dashboard_wasm";

@Module({
	stateFactory: true
})
export default class Group extends VuexModule
{
	private group_data: Map<string, GroupData> = new Map();

	private groups: string[] = [];
	private group_list_end = false;

	get group() {
		return (id: string) => {
			return this.group_data.get(id);
		};
	}
	
	get getGroups() {
		return this.groups;
	}

	get groupListEnd() {
		return this.group_list_end;
	}

	@Mutation
	public setGroup(data: GroupData)
	{
		//push new group
		this.groups.unshift(data.id);

		this.group_data.set(data.id, data);
	}

	@Mutation
	public setGroupData(data: GroupData)
	{
		this.group_data.set(data.id, data);
	}

	@Mutation
	public setGroups(data: GroupData[])
	{
		if (data.length === 0) {
			this.group_list_end = true;
			return;
		}

		if (data.length < 20) {
			this.group_list_end = true;
		}
		
		for (let i = 0; i < data.length; i++) {
			const datum = data[i];

			this.groups.push(datum.id);

			this.group_data.set(datum.id, datum);
		}
	}

	@Action({rawError: true})
	public async fetchGroup(group_id: string)
	{
		try {
			const jwt = await this.context.dispatch("customer/Customer/getJwt", {}, {root: true});

			const data = await get_group(process.env.NUXT_ENV_BASE_URL, jwt, group_id);
			const apps: AppData[] = data.get_apps();

			const group_data: GroupData = {
				id: data.get_id(),
				time: data.get_time(),
				group_name: data.get_name() ?? undefined,
				des: data.get_des() ?? undefined,
				rank: data.get_rank()
			};
			
			this.context.commit("setGroupData", group_data);
			this.context.commit("app/App/setAppsGroup", {group_id, apps}, {root: true});
		} catch (e) {
			//no need to set error
		}
	}

	@Action({rawError: true})
	public async fetchGroups()
	{
		if (this.group_list_end) {
			return;
		}

		const last = this.groups[this.groups.length - 1] ?? "none";
		const last_time = this.group_data.get(last)?.time.toString() ?? "0";

		let groups: GroupData[] = [];

		try {
			const jwt = await this.context.dispatch("customer/Customer/getJwt", {}, {root: true});

			groups = await get_all_groups(process.env.NUXT_ENV_BASE_URL, jwt, last_time, last);
		} catch (e) {
			//no need to set error
		}

		this.context.commit("setGroups", groups);
	}
}