<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col
				cols="12"
				sm="12"
				md="8"
				lg="6"
				:class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}"
				style="max-width: 1300px"
			>
				<h1 class="display-1">Settings</h1>

				<v-form @submit.prevent="changeEmail">
					<v-card flat>
						<v-card-title class="headline">Change email</v-card-title>

						<v-card-text>
							<v-text-field v-model="email" label="New E-Mail" :rules="[rules.required, rules.email]" />
						</v-card-text>

						<v-card-actions>
							<v-spacer />
							<v-btn type="submit" text color="primary">Change E-Mail</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>

				<v-divider />

				<v-form @submit.prevent="updateData">
					<v-card flat>
						<v-card-title class="headline">Update data</v-card-title>

						<v-card-text>
							<v-text-field v-model="first_name" label="Firstname" :rules="[rules.required]" />
							<v-text-field v-model="customer_name" label="Name" :rules="[rules.required]" />
							<v-text-field v-model="company" label="Company" messages="Optional" />
						</v-card-text>

						<v-card-actions>
							<v-spacer />
							<v-btn type="submit" text color="primary">Update data</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>

				<v-divider />

				<v-form @submit.prevent="changePassword">
					<v-card flat>
						<v-card-title class="headline">Change password</v-card-title>

						<v-card-text>
							<v-text-field
								v-model="old_pw"
								label="Old password"
								:type="showPassword ? 'text' : 'password'"
								prepend-icon="mdi-lock"
								:append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
								:rules="[rules.required]"
								@click:append="showPassword = !showPassword"
							/>

							<v-text-field
								v-model="new_pw"
								label="Password"
								:type="showPassword ? 'text' : 'password'"
								prepend-icon="mdi-lock"
								:append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
								:rules="[rules.required]"
								loading
								@click:append="showPassword = !showPassword"
							>
								<template #progress>
									<v-progress-linear
										:color="score.color"
										:value="score.value"
										class="text-right"
										absolute
										height="7"
									/>
								</template>
							</v-text-field>

							<v-text-field
								v-model="new_pw_2"
								label="Confirm password"
								:type="showPassword ? 'text' : 'password'"
								prepend-icon="mdi-lock"
								:append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
								:rules="[rules.required]"
								@click:append="showPassword = !showPassword"
							/>
						</v-card-text>

						<v-card-actions>
							<v-spacer />
							<v-btn text color="primary" type="submit" :loading="pw_change_loading">Change password</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>

				<v-divider />

				<v-expansion-panels flat popout>
					<v-expansion-panel>
						<v-expansion-panel-header>Danger Zone</v-expansion-panel-header>
						<v-expansion-panel-content eager>
							<v-card-title class="headline">Delete account</v-card-title>

							<v-card-text>
								<v-text-field
									v-model="delete_pw"
									label="Password"
									:type="showPassword ? 'text' : 'password'"
									prepend-icon="mdi-lock"
									:append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
									:rules="[rules.required]"
									messages="Please enter your password to delete the account"
									@click:append="showPassword = !showPassword"
								/>
							</v-card-text>

							<v-card-actions>
								<v-spacer />
								<v-btn text color="error" @click="delete_sheet=true">Delete</v-btn>
							</v-card-actions>
						</v-expansion-panel-content>
					</v-expansion-panel>
				</v-expansion-panels>
			</v-col>
		</v-row>

		<v-dialog v-model="dialog" max-width="700">
			<v-card>
				<v-card-title class="headline">Email was successfully updated</v-card-title>
				<v-card-text>Please verify your new email.</v-card-text>

				<v-card-actions>
					<v-spacer />
					<v-btn text @click="dialog=false">close</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>

		<v-dialog v-model="pw_change_dialog" max-width="700">
			<v-card>
				<v-card-title class="headline">Password was successfully changed</v-card-title>

				<v-card-actions>
					<v-spacer />
					<v-btn text @click="pw_change_dialog=false">close</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>

		<v-bottom-sheet v-model="delete_sheet" persistent>
			<v-sheet
				class="text-center"
				height="200px"
			>
				<div class="pa-3">
					<h1 class="display-5">Delete Account</h1>
					<br>

					Do you really want to delete your account?
				</div>

				<v-btn class="mt-6" text color="error" @click="deleteUser">Delete</v-btn>
				<v-btn class="mt-6" text color="primary" @click="delete_sheet = false">Cancel</v-btn>
			</v-sheet>
		</v-bottom-sheet>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {Action, Getter, Mutation} from "nuxt-property-decorator";
import {change_password, delete_customer, update, update_data} from "server_dashboard_wasm";
import {SentcError} from "~/utils/types";
import zxcvbn from "zxcvbn";
import {p} from "~/utils/utils";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
	fetch() {
		this.customer_name = this.getName;
		this.first_name = this.getFirstName;
		this.company = this.getCompany ?? "";
	},
	computed: {
		score() {
			if (this.new_pw.length < 6) {
				return {
					color: "red",
					value: 0
				};
			}

			const result = zxcvbn(this.new_pw);

			switch (result.score) {
				case 0:
					return {
						color: "red",
						value: 10
					};
				case 1:
					return {
						color: "orange",
						value: 25
					};
				case 2:
					return {
						color: "yellow",
						value: 50
					};
				case 3:
					return {
						color: "light-green",
						value: 75
					};
				case 4:
					return {
						color: "light-blue",
						value: 100
					};
			}
		}
	}
})
export default class extends Vue
{
	@Getter("customer/Customer/getEmail")
	private getEmail: string;

	@Getter("customer/Customer/getName")
	private getName: string;

	@Getter("customer/Customer/getFirstName")
	private getFirstName: string;

	@Getter("customer/Customer/getCompany")
	private getCompany?: string;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	@Action("customer/Customer/logout")
	private logout: () => Promise<void>;

	@Action("customer/Customer/setEmail")
	private setEmail: (email: string)=> Promise<void>;

	@Action("customer/Customer/setCustomerData")
	private setCustomerData: (data: {name: string, first_name: string, company?: string})=> Promise<void>;

	@Action("customer/Customer/getFreshJwt")
	private getFreshJwt: (password: string, mfa_token?: string, mfa_recovery?: boolean) => Promise<string>;

	private email = "";

	private customer_name = "";
	private first_name = "";
	private company = "";

	private old_pw = "";
	private new_pw = "";
	private new_pw_2 = "";

	private delete_pw = "";

	private dialog = false;
	private pw_change_dialog = false;
	private showPassword = false;
	private pw_change_loading = false;
	private delete_sheet = false;

	rules = {
		required: (value) => { return !!value || "Required."; },
		email: (value) => {
			const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
			return pattern.test(value) || "Invalid e-mail.";
		}
	};

	private async changeEmail()
	{
		if (!this.email || this.email === "") {
			return;
		}

		try {
			const jwt = await this.getJwt();
			await update(process.env.NUXT_ENV_BASE_URL, jwt, this.email);

			await this.setEmail(this.email);

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

	private async updateData()
	{
		if (!this.customer_name || this.customer_name === "" || !this.first_name || this.first_name === "") {
			return;
		}

		if (!this.company) {
			this.company = "";
		}

		try {
			const jwt = await this.getJwt();

			await update_data(process.env.NUXT_ENV_BASE_URL, jwt, this.customer_name, this.first_name, this.company);

			await this.setCustomerData({name: this.customer_name, first_name: this.first_name, company: this.company});
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}

	private async changePassword()
	{
		if (!this.old_pw || this.old_pw === "" || !this.new_pw || this.new_pw === "") {
			return;
		}

		if (this.new_pw !== this.new_pw_2) {
			this.setMsg("Password and password confirm are not the same");
			return;
		}

		this.pw_change_loading = true;

		try {
			//FixMe mfa
			await change_password(process.env.NUXT_ENV_BASE_URL, this.getEmail, this.old_pw, this.new_pw);

			this.pw_change_dialog = true;
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}

		this.pw_change_loading = false;
	}

	private async deleteUser()
	{
		if (this.getEmail === "") {
			this.setMsg("Not logged in");
			return false;
		}

		if (!this.delete_pw || this.delete_pw === "") {
			return; 
		}

		try {
			//FixMe mfa
			const fresh_jwt = await this.getFreshJwt(this.delete_pw);

			await delete_customer(process.env.NUXT_ENV_BASE_URL, fresh_jwt);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}

			return;
		}

		await this.logout();

		location.replace(p(""));
	}
}
</script>

<style scoped>

</style>