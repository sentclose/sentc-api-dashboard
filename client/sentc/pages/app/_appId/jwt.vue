<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />
		<v-card v-if="app_data && app_data !== {}" align="center" style="min-height: 50vh">
			<v-card-title class="headline">Jwt Key list</v-card-title>

			<div style="overflow-x: auto">
				<div style="width: 1200px">
					<v-simple-table>
						<template #default>
							<thead>
								<tr>
									<th style="width: 290px">Id</th>
									<th>Jwt sign key</th>
									<th>Jwt verify key</th>
									<th>Created time</th>
									<th style="width: 40px">Delete</th>
								</tr>
							</thead>

							<tbody>
								<tr v-for="(jwt_data,i) in app_jwt_data" :key="i">
									<td>{{ jwt_data.jwt_key_id }}</td>
									<td><v-text-field v-model="jwt_data.sign_key" class="txt" readonly append-outer-icon="mdi-content-copy" @click:append-outer="copyKey(jwt_data.sign_key)" /></td>
									<td><v-text-field v-model="jwt_data.verify_key" class="txt" readonly append-outer-icon="mdi-content-copy" @click:append-outer="copyKey(jwt_data.sign_key)" /></td>
									<td>{{ ts(jwt_data.time) }}</td>
									<td style="width: 40px"><v-btn icon color="error" @click="delete_sheet = !delete_sheet; key_to_delete = jwt_data.jwt_key_id"><v-icon>mdi-delete</v-icon></v-btn></td>
								</tr>
							</tbody>
						</template>
					</v-simple-table>
				</div>
			</div>
		</v-card>

		<v-bottom-sheet v-model="delete_sheet" persistent>
			<v-sheet
				class="text-center"
				height="200px"
			>
				<div class="pa-3">
					<h1 class="display-5">Delete jwt key</h1>
					<br>

					Do you really want to delete this key: <b>{{ key_to_delete }}</b> ?
				</div>

				<v-btn class="mt-6" text color="error" @click="deleteJwt">Delete</v-btn>
				<v-btn class="mt-6" text color="primary" @click="delete_sheet = false">Cancel</v-btn>
			</v-sheet>
		</v-bottom-sheet>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {AppDetails, AppJwtData, SentcError} from "~/utils/types";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {getTime} from "~/utils/utils";
import {delete_jwt_keys} from "server_dashboard_wasm/server_dashboard_wasm_cjs";

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
	private key_to_delete = "";

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

	private copyKey(key: string)
	{
		//TODO
	}

	private async deleteJwt()
	{
		const id = this.key_to_delete;

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

		this.key_to_delete = "";
		this.delete_sheet = false;
	}
}
</script>

<style scoped>

</style>