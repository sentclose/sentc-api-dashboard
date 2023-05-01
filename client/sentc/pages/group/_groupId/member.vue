<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-row class="mx-3 mt-3 mb-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
					<h1 class="display-1">Member</h1> <v-spacer />
					<v-btn color="primary" @click="inviteDialog = !inviteDialog">Invite member</v-btn>
				</v-row>

				<v-row :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" class="mb-5">
					<v-col cols="12">
						<v-data-table
							:headers="table_header"
							:items="list.member"
							hide-default-footer
							:items-per-page="-1"
						>
							<template #[`item.joined_time`]="{item}">
								{{ ts(item.joined_time) }}
							</template>

							<template #[`item.rank`]="{item}">
								{{ getRankFromNumber(item.rank) }}
							</template>

							<template #[`item.action`]="{item}">
								<v-btn icon @click="editMember(item)">
									<v-icon>mdi-pencil</v-icon>
								</v-btn>
							</template>
						</v-data-table>
					</v-col>
				</v-row>

				<v-row v-if="!list.end" align="center" justify="center" class="my-5">
					<v-btn color="primary" text @click="loadMore">Load more</v-btn>
				</v-row>
			</v-col>
		</v-row>

		<v-dialog v-model="inviteDialog" max-width="700">
			<v-card>
				<v-card-title class="headline">Invite a new member</v-card-title>

				<v-card-text>
					<v-text-field
						v-model="user_to_invite_id"
						label="User id"
						prepend-icon="mdi-account-circle"
						:rules="[rules.required]"
					/>

					<v-autocomplete
						ref="rank"
						v-model="user_rank"
						messages="Optional"
						:items="group_ranks"
						label="New member rank"
						prepend-icon="mdi-account-circle"
						clearable
					/>
				</v-card-text>

				<v-card-actions>
					<v-spacer />
					<v-btn text color="success" @click="inviteMember">Invite</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import groupAccess from "~/middleware/groupAccess";
import {GroupMember} from "~/store/group/Group";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {getTime} from "~/utils/utils";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {CustomerGroupMemberListItem, SentcError} from "~/utils/types";
import {invite_member} from "server_dashboard_wasm";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
	middleware: [({store, redirect}) => {
		const logged_in = store.getters["customer/Customer/loggedIn"];

		if (logged_in !== 1) {
			return redirect("/login");
		}
	}, groupAccess],
	async fetch() {
		const group_id = this.$route.params.groupId;

		const member = this.groupMember(group_id);

		if (member.member.length === 0) {
			await this.fetchMember(group_id);
		}

		this.list = this.groupMember(group_id);
	}
})
export default class extends Vue
{
	//TODO
	//TODO change member rank
	//TODO kick member

	list: GroupMember = {member: [], end: false};

	inviteDialog = false;

	user_to_invite_id = "";
	user_rank: string | null = null;

	rules = {
		required: (value) => { return !!value || "Required."; }
	};

	group_ranks = ["admin", "moderator", "member"];

	table_header = [
		{text: "First name", align: "start", value: "first_name"},
		{text: "Last name", value: "name"},
		{text: "Email", value: "email"},
		{text: "Group rank", value: "rank"},
		{text: "member since", value: "joined_time"},
		{text: "Action", value: "action"}
	];

	@Getter("group/Group/groupMember")
	private groupMember: (id: string) => GroupMember;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("group/Group/fetchMember")
	private fetchMember: (id: string) => Promise<void>;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	private async loadMore()
	{
		await this.fetchMember(this.$route.params.groupId);
	}

	private async inviteMember()
	{
		if (this.user_to_invite_id === "") {
			return;
		}

		try {
			const jwt = await this.getJwt();

			await invite_member(process.env.NUXT_ENV_BASE_URL, jwt, this.$route.params.groupId, this.user_to_invite_id, this.getRankNumber());

			this.inviteDialog = false;

			this.$forceUpdate();
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}

	private editMember(item: CustomerGroupMemberListItem)
	{
		console.log(item.user_id);
	}

	private getRankNumber()
	{
		switch (this.user_rank) {
			case "admin": return 1;
			case "moderator": return 2;
			case "member": return 4;
			default: return 4;
		}
	}

	private getRankFromNumber(rank)
	{
		switch (rank) {
			case 0: return "creator";
			case 1: return "admin";
			case 2: return "moderator";
			default: return "member";
		}
	}
}
</script>

<style scoped>

</style>