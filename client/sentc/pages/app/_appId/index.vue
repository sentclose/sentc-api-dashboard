<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />

		<v-card v-if="app_data && app_data !== {}">
			<v-card-title class="headline">
				{{ app_data.identifier }}
			</v-card-title>

			<v-card-text>
				Created at: {{ ts(Number(app_data.time)) }}
			</v-card-text>

			<v-card-text>
				Welcome to the app dashboard. This app can be configured on the left menu. <br>

				In JWT, new jwt keys can be added ore old once deleted. <br>

				In App access token can a new secret and public token pair created. The old tokens are invalid.
			</v-card-text>
		</v-card>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {AppDetails} from "~/utils/types";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {getTime} from "~/utils/utils";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
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

	@Mutation("app/App/removeJwtData")
	private removeJwtData: (data: {jwt_id: string, app_id: string}) => void;

	@Action("app/App/fetchDetails")
	private fetchDetails: (app_id: string) => Promise<void>;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	/*
	TODO
		- add jwt keys
		- renew token
		- delete app
		- update app call
		- show jwt keys in a non edit text field
	 */
}
</script>

<style scoped>

.txt ::v-deep(.v-input__slot::before) {
	border-style: none !important;
}
</style>