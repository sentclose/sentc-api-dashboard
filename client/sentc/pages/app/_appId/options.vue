<template>
	<div>
		<ErrorEvent />
		<v-card v-if="app_data && app_data !== {}" style="min-height: 50vh" flat>
			<v-card-title class="headline"><v-icon>mdi-cog</v-icon> App options</v-card-title>

			<v-card-text>
				<AppOptions ref="options" :data="app_data.options" />
			</v-card-text>

			<v-card-actions>
				<v-spacer />

				<v-btn color="primary" text @click="updateOptions">Update</v-btn>
			</v-card-actions>
		</v-card>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {AppDetails} from "~/utils/types";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import AppOptions from "~/components/App/AppOptions.vue";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {AppOptions, ErrorEvent},
	layout: "app",
	async fetch() {
		this.app_id = this.$route.params?.appId;

		if (!this.app_id || this.app_id === "") {
			return this.$router.push("/");
		}

		this.app_data = this.getAppDetails(this.app_id);

		// eslint-disable-next-line eqeqeq
		if (!this.app_data || this.app_data == {}) {
			await this.fetchDetails(this.app_id);

			this.app_data = this.getAppDetails(this.app_id);
		}
	}
})
export default class extends Vue
{
	private app_id = "";

	//@ts-ignore
	private app_data: AppDetails = {};

	@Getter("app/App/appDetails")
	private getAppDetails: (id: string) => AppDetails;

	@Action("app/App/fetchDetails")
	private fetchDetails: (app_id: string) => Promise<void>;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	private async updateOptions()
	{

	}
}

</script>

<style scoped>

</style>