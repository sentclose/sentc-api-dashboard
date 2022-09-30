<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />

		<v-row v-if="app_data && app_data !== {}" justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-card>
					<v-card-title class="headline">
						{{ app_data.identifier }}
					</v-card-title>

					<v-card-text>
						Created at: {{ ts(app_data.time) }}
					</v-card-text>

					<v-expansion-panels popout>
						<v-expansion-panel>
							<v-expansion-panel-header expand-icon="mdi-chevron-down">
								<span><v-icon>mdi-key</v-icon> Jwt keys</span>
							</v-expansion-panel-header>
							<v-expansion-panel-content>
								<div style="overflow-x: auto">
									<div style="width: 1200px">
										<v-simple-table>
											<template #default>
												<thead>
													<tr>
														<th>Jwt sign key</th>
														<th>Jwt verify key</th>
														<th>Created time</th>
														<th style="width: 40px">Delete</th>
													</tr>
												</thead>

												<tbody>
													<tr v-for="(jwt_data,i) in app_jwt_data" :key="i">
														<td><v-text-field v-model="jwt_data.sign_key" class="txt" readonly append-outer-icon="mdi-content-copy" @click:append-outer="copyKey(jwt_data.sign_key)" /></td>
														<td><v-text-field v-model="jwt_data.verify_key" class="txt" readonly append-outer-icon="mdi-content-copy" @click:append-outer="copyKey(jwt_data.sign_key)" /></td>
														<td>{{ ts(jwt_data.time) }}</td>
														<td style="width: 40px"><v-btn icon color="error"><v-icon>mdi-delete</v-icon></v-btn></td>
													</tr>
												</tbody>
											</template>
										</v-simple-table>
									</div>
								</div>
							</v-expansion-panel-content>
						</v-expansion-panel>
					</v-expansion-panels>
				</v-card>
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
		try {
			const jwt = await this.getJwt();

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

	private copyKey(key: string)
	{
		//TODO
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