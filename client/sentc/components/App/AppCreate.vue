<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" align="center" class="mt-3">
			<v-img :src="p('Sentc.png')" max-width="200" />
		</v-row>

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-form @submit.prevent="createApp">
					<v-card flat>
						<v-card-title class="headline">Create an app</v-card-title>

						<v-card-text style="max-width: 300px">
							<v-text-field v-model="identifier" label="App name" messages="optional" />
						</v-card-text>

						<v-divider />

						<v-card-title class="title">Options</v-card-title>
						<v-card-text style="color: black">
							<p>
								The following options are optional. The default options require an own backend, <br>
								for user registration, user delete, group create and group delete.
							</p>
							<p>
								Files are disabled by default. If you need to handle larger files, just enable the file storage.
							</p>
						</v-card-text>

						<v-expansion-panels popout flat>
							<v-expansion-panel>
								<v-expansion-panel-header expand-icon="mdi-chevron-down">
									<span><v-icon>mdi-cog</v-icon> App options</span>
								</v-expansion-panel-header>
								<v-expansion-panel-content eager>
									<AppOptions ref="options" />
								</v-expansion-panel-content>
							</v-expansion-panel>
							<v-expansion-panel>
								<v-expansion-panel-header expand-icon="mdi-chevron-down">
									<span><v-icon>mdi-file</v-icon> App file options</span>
								</v-expansion-panel-header>
								<v-expansion-panel-content eager>
									<AppFileOptions ref="file_options" />
								</v-expansion-panel-content>
							</v-expansion-panel>
							<v-expansion-panel>
								<v-expansion-panel-header expand-icon="mdi-chevron-down">
									<span><v-icon>mdi-account-group</v-icon> App group options</span>
								</v-expansion-panel-header>
								<v-expansion-panel-content eager>
									<AppGroupOptions ref="group_options" />
								</v-expansion-panel-content>
							</v-expansion-panel>
						</v-expansion-panels>

						<v-divider class="mt-5" />

						<v-card-actions>
							<v-spacer />

							<v-btn type="submit" text color="success">Create</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-col>
		</v-row>

		<v-dialog v-model="createDialog" max-width="700" persistent>
			<v-card>
				<v-card-title class="headline">App was successfully created</v-card-title>
				<v-card-title class="title">The app tokens are listed below. The tokens cannot be shown again.</v-card-title>

				<v-card-text>
					<div style="overflow-x: auto">
						<div style="width: 650px">
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

										<tr>
											<td class="table_des">Jwt sign key</td>
											<td><v-text-field v-model="jwt_sign_key" class="mt-2 txt" readonly /></td>
										</tr>

										<tr>
											<td class="table_des">Jwt verify key</td>
											<td><v-text-field v-model="jwt_verify_key" class="mt-2 txt" readonly /></td>
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

					<v-btn text color="success" @click="createDialog = false">Done</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import AppOptions from "~/components/App/AppOptions.vue";
import {Action, Mutation, Prop} from "nuxt-property-decorator";
import {
	AppData,
	AppFileOptions as AppFileOptionsType,
	AppGroupOptions as AppGroupOptionsType,
	AppOptions as AppOptionsType,
	SentcError
} from "~/utils/types";
import AppFileOptions from "~/components/App/AppFileOptions.vue";
import {app_create_app, AppRegisterOutput} from "server_dashboard_wasm";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {p} from "~/utils/utils";
import AppGroupOptions from "~/components/App/AppGroupOptions.vue";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {AppGroupOptions, ErrorEvent, AppFileOptions, AppOptions}
})
export default class AppCreate extends Vue
{
	@Prop({required: true})
	private create: boolean;

	@Prop({required: true})
	private group_id: string | undefined;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	@Mutation("app/App/setApp")
	private setApp: (data: { app: AppData, group_id?: string }) => void;

	private identifier = "";

	private secret_token = "";
	private public_token = "";
	private jwt_sign_key = "";
	private jwt_verify_key = "";

	private createDialog = false;

	private p(item: string)
	{
		return p(item);
	}

	private async createApp()
	{
		//@ts-ignore
		const options: AppOptionsType = this.$refs.options.options;

		//@ts-ignore
		const file_options: AppFileOptionsType = this.$refs.file_options.getOptions();

		if (!file_options) {
			return;
		}

		//@ts-ignore
		const group_options: AppGroupOptionsType = this.$refs.group_options.getOptions();

		try {
			const jwt = await this.getJwt();

			const out: AppRegisterOutput = await app_create_app(process.env.NUXT_ENV_BASE_URL, jwt, this.identifier, options, file_options, group_options, this.group_id);

			this.secret_token = out.get_secret_token();
			this.public_token = out.get_public_token();
			this.jwt_sign_key = out.get_jwt_sign_key();
			this.jwt_verify_key = out.get_jwt_verify_key();

			this.createDialog = true;

			const app_data: AppData = {
				time: Date.now(),
				identifier: this.identifier ? this.identifier : "unnamed",
				id: out.get_app_id()
			};

			this.setApp({app: app_data, group_id: this.group_id});
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
SENTC_JWT_SIGN_KEY=${this.jwt_sign_key}
SENTC_JWT_VERIFY_KEY=${this.jwt_verify_key}
		`], {type: "application/octet-stream"});

		const elem = window.document.createElement("a");
		elem.href = window.URL.createObjectURL(blob);
		elem.download = ".env";
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