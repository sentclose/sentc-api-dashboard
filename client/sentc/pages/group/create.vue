<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-form @submit.prevent="createGroup">
					<v-card flat>
						<v-card-title class="headline">Create a group</v-card-title>

						<v-card-text style="max-width: 300px">
							<v-text-field v-model="group_name" label="Group name" messages="optional" />
						</v-card-text>

						<v-card-text style="max-width: 300px">
							<v-text-field v-model="group_des" label="Group description" messages="optional" />
						</v-card-text>

						<v-card-actions>
							<v-spacer />

							<v-btn type="submit" text color="success">Create group</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-col>
		</v-row>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {Action, Mutation} from "nuxt-property-decorator";
import {GroupData, SentcError} from "~/utils/types";
import {create_group} from "server_dashboard_wasm";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent}
})
export default class extends Vue
{
	group_name = "";
	group_des = "";

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Mutation("group/Group/setGroup")
	private setGroup: (data: GroupData) => void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	private async createGroup()
	{
		try {
			const jwt = await this.getJwt();

			const group_id = await create_group(process.env.NUXT_ENV_BASE_URL, jwt, this.group_name, this.group_des);

			const group_data: GroupData = {
				group_name: this.group_name ? this.group_name : "unnamed",
				des: this.group_des ? this.group_des : "",
				id: group_id,
				time: Date.now().toString(),
				rank: 0
			};

			this.setGroup(group_data);

			return this.$router.push(`/group/${group_id}`);
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