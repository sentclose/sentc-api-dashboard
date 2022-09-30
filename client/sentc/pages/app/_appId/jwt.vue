<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />
		<v-card v-if="app_data && app_data !== {}" style="min-height: 50vh" flat>
			<v-card-title class="headline">
				Jwt Key list
				<v-spacer />

				<v-tooltip bottom color="primary">
					<template #activator="{on,attrs}">
						<v-btn
							v-bind="attrs"
							icon
							color="primary"
							v-on="on"
							@click="createJwtKey"
						>
							<v-icon>mdi-plus</v-icon>
						</v-btn>
					</template>
					<span>Create new jwt key</span>
				</v-tooltip>
			</v-card-title>

			<div style="overflow-x: auto">
				<div style="width: 1200px">
					<v-simple-table>
						<template #default>
							<thead>
								<tr>
									<th style="width: 290px">Id</th>
									<th>Jwt sign key</th>
									<th class="col-copy" />
									<th>Jwt verify key</th>
									<th class="col-copy" />
									<th>Created time</th>
									<th style="width: 40px">Delete</th>
								</tr>
							</thead>

							<tbody>
								<tr v-for="(jwt_data,i) in app_jwt_data" :key="i">
									<td>{{ jwt_data.jwt_key_id }}</td>

									<td><v-text-field v-model="jwt_data.sign_key" class="txt" readonly /></td>
									<td class="col-copy">
										<v-tooltip bottom>
											<template #activator="{on,attrs}">
												<v-btn
													v-bind="attrs"
													icon
													v-on="on"
													@click="copyKey(jwt_data.sign_key)"
												>
													<v-icon>mdi-content-copy</v-icon>
												</v-btn>
											</template>
											<span>Copy key</span>
										</v-tooltip>
									</td>

									<td><v-text-field v-model="jwt_data.verify_key" class="txt" readonly /></td>
									<td class="col-copy">
										<v-tooltip bottom>
											<template #activator="{on,attrs}">
												<v-btn
													v-bind="attrs"
													icon
													v-on="on"
													@click="copyKey(jwt_data.verify_key)"
												>
													<v-icon>mdi-content-copy</v-icon>
												</v-btn>
											</template>
											<span>Copy key</span>
										</v-tooltip>
									</td>
									<td>{{ ts(jwt_data.time) }}</td>
									<td style="width: 40px">
										<v-tooltip bottom color="error">
											<template #activator="{on,attrs}">
												<v-btn
													v-bind="attrs"
													icon
													color="error"
													v-on="on"
													@click="delete_sheet = !delete_sheet; key_to_delete = jwt_data.jwt_key_id"
												>
													<v-icon>mdi-delete</v-icon>
												</v-btn>
											</template>
											<span>Delete jwt key</span>
										</v-tooltip>
									</td>
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
import {copyToClipboard, getTime} from "~/utils/utils";
import {delete_jwt_keys, new_jwt_keys} from "server_dashboard_wasm/server_dashboard_wasm_cjs";
import {AppJwtRegisterOutput} from "server_dashboard_wasm/server_dashboard_wasm";

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

	@Mutation("app/App/setAppJwtData")
	private pushAppJwtData: (data: {jwt_data: AppJwtData, id: string}) => void;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	private copyKey(key: string)
	{
		return copyToClipboard(key);
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

		for (let i = 0; i < this.app_jwt_data.length; i++) {
			if (this.app_jwt_data[i].jwt_key_id === id) {
				this.app_jwt_data.splice(i, 1);
				break;
			}
		}

		this.key_to_delete = "";
		this.delete_sheet = false;
	}

	private async createJwtKey()
	{
		try {
			const jwt = await this.getJwt();

			const out: AppJwtRegisterOutput = await new_jwt_keys(process.env.NUXT_ENV_BASE_URL, jwt, this.app_id);

			const jwt_data: AppJwtData = {
				jwt_key_id: out.get_jwt_id(),
				sign_key: out.get_jwt_sign_key(),
				verify_key: out.get_jwt_verify_key(),
				jwt_alg: out.get_jwt_alg(),
				time: Date.now()
			};

			this.pushAppJwtData({jwt_data, id: this.app_id});

			this.app_jwt_data.unshift(jwt_data);
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
.txt ::v-deep(.v-input__slot::before) {
	border-style: none !important;
}

.txt {
	margin-top: 10px;
}

.col-copy{
	width: 40px;
	margin: 0;
	padding: 0;
}
</style>