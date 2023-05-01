<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-form @submit.prevent="updateGroup">
					<v-card flat>
						<v-card-title class="headline">Update Group</v-card-title>

						<v-card-text style="max-width: 300px">
							<v-text-field v-model="group_name" label="Group name" messages="optional" clearable />
						</v-card-text>

						<v-card-text style="max-width: 300px">
							<v-text-field v-model="group_des" label="Group description" messages="optional" clearable />
						</v-card-text>

						<v-card-actions>
							<v-spacer />

							<v-btn type="submit" text color="success">Update group</v-btn>
						</v-card-actions>

						<v-divider class="mt-3" />

						<v-expansion-panels flat popout>
							<v-expansion-panel>
								<v-expansion-panel-header>Danger Zone</v-expansion-panel-header>
								<v-expansion-panel-content eager>
									<v-btn text color="error" @click="delete_sheet = !delete_sheet">Delete Group</v-btn>
								</v-expansion-panel-content>
							</v-expansion-panel>
						</v-expansion-panels>
					</v-card>
				</v-form>
			</v-col>
		</v-row>

		<v-bottom-sheet v-model="delete_sheet" persistent>
			<v-sheet
				class="text-center"
				height="200px"
			>
				<div class="pa-3">
					<h1 class="display-5">Delete group</h1>
					<br>

					Do you really want to delete this group?
				</div>

				<v-btn class="mt-6" text color="error" @click="deleteGroup">Delete</v-btn>
				<v-btn class="mt-6" text color="primary" @click="delete_sheet = false">Cancel</v-btn>
			</v-sheet>
		</v-bottom-sheet>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import groupAccess from "~/middleware/groupAccess";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {GroupData, SentcError} from "~/utils/types";
import {delete_group, update_group} from "server_dashboard_wasm";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
	middleware: [({store, redirect}) => {
		const logged_in = store.getters["customer/Customer/loggedIn"];

		if (logged_in !== 1) {
			return redirect("/login");
		}
	}, groupAccess],
	fetch() {
		const group_id = this.$route.params.groupId;

		const group = this.getGroup(group_id);

		this.group_name = group.group_name;
		this.group_des = group.des;
	}
})
export default class extends Vue
{
	private delete_sheet = false;

	group_name = "";
	group_des = "";

	@Getter("group/Group/group")
	private getGroup: (id: string)=> GroupData;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Mutation("group/Group/updateGroup")
	private updateGroupStore: (data: {id: string, name?: string, des?: string}) => void;

	@Mutation("group/Group/removeGroup")
	private removeGroup: (id: string) => void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	private async updateGroup()
	{
		try {
			const jwt = await this.getJwt();

			await update_group(process.env.NUXT_ENV_BASE_URL, jwt, this.$route.params.groupId, this.group_name, this.group_des);

			this.updateGroupStore({id: this.$route.params.groupId, name: this.group_name, des: this.group_des});
			
			return this.$router.push(`/group/${this.$route.params.groupId}`);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}

	private async deleteGroup()
	{
		try {
			const jwt = await this.getJwt();

			await delete_group(process.env.NUXT_ENV_BASE_URL, jwt, this.$route.params.groupId);

			this.removeGroup(this.$route.params.groupId);

			return this.$router.push("/");
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}
}
</script>

<style scoped>

</style>