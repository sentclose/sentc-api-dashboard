<template>
	<div v-if="!$fetchState.pending">
		<ErrorEvent />

		<v-card v-if="app_data && app_data !== {}" style="min-height: 50vh" flat>
			<v-card-title class="headline"><v-icon left>mdi-key-variant</v-icon> Renew app token</v-card-title>
			<v-card-text class="pb-3">
				<p>
					Sentc api doesn't know the tokens because they are hashed. If you forgot the tokens, you have to create new tokens. <br>
					After renew the old tokens are invalid and can't be used anymore.
				</p>

				<v-divider />
			</v-card-text>

			<v-card-actions class="mt-0">
				<v-spacer />
				<v-btn text color="primary" @click="sheet = true">Renew tokens</v-btn>
			</v-card-actions>
		</v-card>

		<v-bottom-sheet v-model="sheet" persistent>
			<v-sheet
				class="text-center"
				height="200px"
			>
				<div class="pa-3">
					<h1 class="display-5">Renew the tokens</h1>
					<br>

					Do you really want to renew the tokens?
				</div>

				<v-btn class="mt-6" text color="error" @click="renew">Renew</v-btn>
				<v-btn class="mt-6" text color="primary" @click="sheet = false">Cancel</v-btn>
			</v-sheet>
		</v-bottom-sheet>

		<v-dialog v-model="dialog" max-width="750" persistent>
			<v-card>
				<v-card-title class="headline">App token were successfully renewed</v-card-title>
				<v-card-title class="title">The app tokens are listed below. The tokens cannot be shown again.</v-card-title>

				<v-card-text>
					<div style="overflow-x: auto">
						<div style="width: 700px">
							<v-simple-table>
								<template #default>
									<tbody>
										<tr>
											<td class="table_des">Secret token</td>
											<td><v-text-field v-model="secret_token" class="mt-2 txt" readonly /></td>
										</tr>

										<tr>
											<td class="table_des">Public token</td>
											<td><v-text-field v-model="public_token" class="mt-2 txt" readonly /></td>
										</tr>
									</tbody>
								</template>
							</v-simple-table>
						</div>
					</div>
				</v-card-text>

				<v-card-actions>
					<v-btn text color="primary" @click="copyEnv">Export as .env</v-btn>

					<v-spacer />

					<v-btn text color="success" @click="dialog = false">Done</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {AppDetails, SentcError} from "~/utils/types";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {AppTokenRenewOutput, renew_token} from "server_dashboard_wasm";

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

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	private secret_token = "";
	private public_token = "";

	private dialog = false;
	private sheet = false;

	private async renew()
	{
		try {
			const jwt = await this.getJwt();

			const out: AppTokenRenewOutput = await renew_token(process.env.NUXT_ENV_BASE_URL, jwt, this.app_id);

			this.secret_token = out.get_secret_token();
			this.public_token = out.get_public_token();

			this.sheet = false;
			this.dialog = true;
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}

	private copyEnv()
	{
		const blob = new Blob([`SENTC_SECRET_TOKEN=${this.secret_token}
SENTC_PUBLIC_TOKEN=${this.public_token}
		`], {type: "application/octet-stream"});

		const elem = window.document.createElement("a");
		elem.href = window.URL.createObjectURL(blob);
		elem.download = `.env`;
		document.body.appendChild(elem);
		elem.click();
		document.body.removeChild(elem);
	}
}
</script>

<style scoped>
.table_des{
	width: 150px;
}

.txt ::v-deep(.v-input__slot::before) {
	border-style: none !important;
}
</style>