<template>
	<div>
		<ErrorEvent />

		<v-card v-if="app_data && app_data !== {}" style="min-height: 50vh" flat>
			<v-card-title class="headline"><v-icon left>mdi-file</v-icon> App File options</v-card-title>

			<v-card-text>
				<AppFileOptions ref="file_options" :data="app_data.file_options" />
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
import {
	AppDetails,
	AppFileOptions as AppFileOptionsType,
	SentcError
} from "~/utils/types";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import AppFileOptions from "~/components/App/AppFileOptions.vue";
import {app_update_file_options} from "server_dashboard_wasm";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {AppFileOptions, ErrorEvent},
	layout: "app",
	async fetch() {
		this.app_id = this.$route.params?.appId;

		if (!this.app_id || this.app_id === "") {
			return this.$router.push("/");
		}

		this.app_data = this.getAppDetails(this.app_id);

		// eslint-disable-next-line eqeqeq
		if (!this.app_data || this.app_data?.id == undefined) {
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

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Mutation("app/App/setAppFileOptions")
	private setAppFileOptions: (data: {id: string, options: AppFileOptionsType})=>void;

	private async updateOptions()
	{
		//@ts-ignore
		const file_options: AppFileOptionsType = this.$refs.file_options.getOptions();

		if (!file_options) {
			return;
		}

		try {
			const jwt = await this.getJwt();

			await app_update_file_options(process.env.NUXT_ENV_BASE_URL, jwt, this.app_id, file_options);

			this.setAppFileOptions({id: this.app_id, options: file_options});
			this.app_data.file_options = file_options;
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