<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
					<v-col cols="12" sm="10" md="8" lg="6">
						<v-card>
							<v-card-title class="headline">
								{{ app_data.identifier }}
							</v-card-title>

							<v-card-text>
								Created at: {{ ts(app_data.time) }}
							</v-card-text>
						</v-card>
					</v-col>
				</v-row>

				<v-row :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
					<v-col cols="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
						<v-card>
							<v-card-title class="headline">Created Jwt</v-card-title>
						</v-card>

						<v-card v-for="(jwt_data,i) in app_jwt_data" :key="i" class="mt-3">
							<v-card-text>
								Jwt verify key {{ jwt_data.verify_key }}
							</v-card-text>

							<v-card-text>
								Jwt sign key {{ jwt_data.sign_key }}
							</v-card-text>

							<v-card-text>
								Jwt created: {{ ts(jwt_data.time) }}
							</v-card-text>

							<v-card-actions>
								<v-spacer />

								<v-btn color="error" @click="delete_sheet = true"><v-icon left>mdi-delete</v-icon> Delete jwt</v-btn>

								<v-bottom-sheet v-model="delete_sheet" persistent>
									<v-sheet
										class="text-center"
										height="200px"
									>
										<div class="pa-3">
											<h1 class="display-5">Confirm</h1>
											<br>

											Do you really want to delete this jwt?
										</div>

										<v-btn class="mt-6" text color="error" @click="deleteJwt(jwt_data.jwt_key_id)">Delete</v-btn>
										<v-btn class="mt-6" text color="primary" @click="delete_sheet = false">Cancel</v-btn>
									</v-sheet>
								</v-bottom-sheet>
							</v-card-actions>
						</v-card>
					</v-col>
				</v-row>
			</v-col>
		</v-row>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {AppDetails, AppJwtData, SentcError} from "~/utils/types";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {getTime} from "~/utils/utils";
import {delete_jwt_keys} from "server_dashboard_wasm/server_dashboard_wasm_cjs";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
	async fetch() {
		this.app_id = this.$router.params?.app_id;

		if (!this.app_id || this.app_id === "") {
			return this.$router.push("/");
		}

		this.app_data = this.getAppDetails(this.app_id);

		// eslint-disable-next-line eqeqeq
		if (!this.app_data || this.app_data == {}) {
			await this.fetchDetails(this.app_id);

			this.app_data = this.getAppDetails(this.app_id);
		}

		//now get the jwt data
		this.app_jwt_data = this.jwtData(this.app_id);

		// eslint-disable-next-line eqeqeq
		if (!this.app_jwt_data || this.app_jwt_data == {}) {
			await this.fetchJwtData(this.app_id);

			this.app_jwt_data = this.jwtData(this.app_id);
		}
	}
})
export default class extends Vue
{
	private app_id = "";

	//@ts-ignore
	private app_data: AppDetails = {};

	private app_jwt_data: AppJwtData[] = [];

	private delete_sheet = false;

	@Getter("app/App/appDetails")
	private getAppDetails: (id: string) => AppDetails;

	@Getter("app/App/jwtData")
	private jwtData: (id: string) => AppJwtData;

	@Mutation("app/App/removeJwtData")
	private removeJwtData: (data: {jwt_id: string, app_id: string}) => void;

	@Action("app/App/fetchJwtData")
	private fetchJwtData: (id: string) => Promise<void>;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	@Action("app/App/fetchDetails")
	private fetchDetails: (app_id: string) => Promise<void>;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	private async deleteJwt(id: string)
	{
		const jwt = await this.getJwt();

		try {
			await delete_jwt_keys(process.env.NUXT_ENV_BASE_URL, jwt, this.app_id, id);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}

			return;
		}

		this.removeJwtData({app_id: this.app_id, jwt_id: id});
	}

	/*
	TODO
		- add jwt keys
		- renew token
		- delete app
	 */
}
</script>

<style scoped>

</style>