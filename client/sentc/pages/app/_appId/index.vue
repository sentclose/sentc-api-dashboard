<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />

		<v-card v-if="app_data && app_data !== {}" flat>
			<v-card-title class="headline">
				<span v-if="!edit_identifier">
					{{ app_data.identifier ? app_data.identifier : "unnamed" }}

					<v-tooltip bottom>
						<template #activator="{on,attrs}">
							<v-btn
								v-bind="attrs"
								class="ml-3"
								icon
								right
								v-on="on"
								@click="edit_identifier = !edit_identifier"
							><v-icon>mdi-pencil</v-icon></v-btn>
						</template>
						<span>Edit identifier</span>
					</v-tooltip>

				</span>

				<span v-if="edit_identifier" class="d-flex">
					<v-text-field v-model="new_identifier" label="App identifier" />

					<v-tooltip bottom color="success">
						<template #activator="{on,attrs}">
							<v-btn
								color="success"
								v-bind="attrs"
								class="mt-3"
								icon
								right
								v-on="on"
								@click="updateIdentifier"
							><v-icon>mdi-plus</v-icon></v-btn>
						</template>
						<span>Set new identifier</span>
					</v-tooltip>

					<v-tooltip bottom color="error">
						<template #activator="{on,attrs}">
							<v-btn
								color="error"
								v-bind="attrs"
								class="mt-3"
								icon
								right
								v-on="on"
								@click="edit_identifier = !edit_identifier"
							><v-icon>mdi-minus</v-icon></v-btn>
						</template>
						<span>Cancel Edit identifier</span>
					</v-tooltip>
				</span>
			</v-card-title>

			<v-card-text>
				Created at: {{ ts(Number(app_data.time)) }}
			</v-card-text>

			<v-card-text>
				Welcome to the app dashboard. This app can be configured on the left menu. <br>

				In JWT, new jwt keys can be added ore old once deleted. <br>

				In App access token can a new secret and public token pair created. The old tokens are invalid.

				<v-divider class="mt-3" />
			</v-card-text>

			<v-expansion-panels flat popout>
				<v-expansion-panel>
					<v-expansion-panel-header>Danger Zone</v-expansion-panel-header>
					<v-expansion-panel-content eager>
						<v-btn text color="error" @click="delete_sheet = !delete_sheet">Delete app</v-btn>
					</v-expansion-panel-content>
				</v-expansion-panel>
			</v-expansion-panels>
		</v-card>

		<v-bottom-sheet v-model="delete_sheet" persistent>
			<v-sheet
				class="text-center"
				height="200px"
			>
				<div class="pa-3">
					<h1 class="display-5">Delete app</h1>
					<br>

					Do you really want to delete this app?
				</div>

				<v-btn class="mt-6" text color="error" @click="deleteApp">Delete</v-btn>
				<v-btn class="mt-6" text color="primary" @click="delete_sheet = false">Cancel</v-btn>
			</v-sheet>
		</v-bottom-sheet>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {AppDetails, SentcError} from "~/utils/types";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {getTime} from "~/utils/utils";
import {app_delete, app_update} from "server_dashboard_wasm";

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

		this.new_identifier = this.app_data.identifier;
	}
})
export default class extends Vue
{
	private app_id = "";

	//@ts-ignore
	private app_data: AppDetails = {};

	private edit_identifier = false;

	private new_identifier = "";

	private delete_sheet = false;

	@Getter("app/App/appDetails")
	private getAppDetails: (id: string) => AppDetails;

	@Mutation("app/App/removeJwtData")
	private removeJwtData: (data: {jwt_id: string, app_id: string}) => void;

	@Action("app/App/fetchDetails")
	private fetchDetails: (app_id: string) => Promise<void>;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Mutation("app/App/setAppIdentifier")
	private setAppIdentifier: (data: {id: string, identifier: string})=>void;

	@Mutation("app/App/removeApp")
	private removeApp: (id: string) => void;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	private async updateIdentifier()
	{
		if (this.new_identifier === this.app_data.identifier) {
			this.edit_identifier = false;
			return;
		}

		try {
			const jwt = await this.getJwt();

			await app_update(process.env.NUXT_ENV_BASE_URL, jwt, this.app_id, this.new_identifier);

			this.setAppIdentifier({id: this.app_id, identifier: this.new_identifier});
			this.app_data.identifier = this.new_identifier;
			this.edit_identifier = false;
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}

	private async deleteApp()
	{
		try {
			const jwt = await this.getJwt();

			await app_delete(process.env.NUXT_ENV_BASE_URL, jwt, this.app_id);

			this.removeApp(this.app_id);

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