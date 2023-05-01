/**
 * @author Jörn Heinemann <joernheinemann@gmx.de>
 * @since 2023/04/30
 */
import {Action, Module, Mutation, VuexModule} from "vuex-module-decorators";
import {AppData, CustomerGroupMemberListItem, CustomerList, GroupData, GroupUserListItem} from "~/utils/types";
import {get_all_groups, get_group, get_member_list} from "server_dashboard_wasm";

export interface GroupMember {
	member: CustomerGroupMemberListItem[],
	end: boolean
}

@Module({
	stateFactory: true
})
export default class Group extends VuexModule
{
	private group_data: Map<string, GroupData> = new Map();

	private group_member: Map<string, GroupMember> = new Map();

	private groups: string[] = [];
	private group_list_end = false;

	get group() {
		return (id: string) => {
			return this.group_data.get(id);
		};
	}

	get groupMember() {
		return (id: string) => {
			if (!this.group_member.has(id)) {
				this.group_member.set(id, {
					member: [],
					end: false
				});
			}

			return this.group_member.get(id);
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
	public updateGroup(data: {id: string, name?: string, des?: string})
	{
		const {id, name, des} = data;

		const group = this.group_data.get(id);

		if (!group) {
			return;
		}

		group.group_name = name;
		group.des = des;
	}

	@Mutation
	public removeGroup(id: string)
	{
		for (let i = 0; i < this.groups.length; i++) {
			if (this.groups[i] === id) {
				this.groups.splice(i, 1);
			}
		}

		this.group_data.delete(id);
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
	
	@Mutation
	public setMember(data: {id: string, member: CustomerGroupMemberListItem[]})
	{
		const {member, id} = data;

		if (!this.group_member.has(id)) {
			this.group_member.set(id, {
				member: [],
				end: false
			});
		}

		const group_member = this.group_member.get(id);

		if (member.length === 0) {
			group_member.end = true;
			return;
		}

		if (member.length < 20) {
			group_member.end = true;
		}

		group_member.member.push(...member);
	}

	@Mutation
	public changeRank(data: {id: string, user_id: string, new_rank: number})
	{
		const {id, new_rank, user_id} = data;

		const group_member = this.group_member.get(id);

		for (let i = 0; i < group_member.member.length; i++) {
			const item = group_member.member[i];

			if (item.user_id === user_id) {
				item.rank = new_rank;
				return;
			}
		}
	}

	@Mutation
	public removeMember(data: {id: string, user_id: string})
	{
		const {id, user_id} = data;

		const group_member = this.group_member.get(id);

		for (let i = 0; i < group_member.member.length; i++) {
			const item = group_member.member[i];

			if (item.user_id === user_id) {
				group_member.member.splice(i, 1);
				return;
			}
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

	@Action({rawError: true})
	public async fetchMember(id: string)
	{
		const member: GroupMember = this.context.getters["groupMember"](id);

		if (member.end) {
			return;
		}

		const member_fetch: CustomerGroupMemberListItem[] = [];

		try {
			const jwt = await this.context.dispatch("customer/Customer/getJwt", {}, {root: true});

			const last = member.member[member.member.length - 1];
			const last_time = last?.joined_time ?? "0";
			const last_id = last?.user_id ?? "none";

			//@ts-ignore
			const fetch = await get_member_list(process.env.NUXT_ENV_BASE_URL, jwt, id, last_time, last_id);

			const customer_data: CustomerList[] = fetch.get_customer();
			const group_member: GroupUserListItem[] = fetch.get_member();

			for (let i = 0; i < customer_data.length; i++) {
				const item = customer_data[i];

				const find = group_member.find((value) => {
					return value.user_id === item.id;
				});

				member_fetch.push({
					user_id: item.id,
					name: item.name,
					first_name: item.first_name,
					email: item.email,
					rank: find.rank,
					joined_time: find.joined_time
				});
			}
		} catch (e) {
			//no need to set error
		}

		this.context.commit("setMember", {id, member: member_fetch});
	}
}