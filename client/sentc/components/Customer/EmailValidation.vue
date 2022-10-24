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
				<div v-if="part === 0">
					<v-form @submit.prevent="validateEmail">
						<v-card flat>
							<v-card-title class="headline">Email validation</v-card-title>

							<v-card-text>
								<v-text-field v-model="token" label="Email token" :rules="[rules.required]" />
							</v-card-text>

							<v-card-actions>
								<v-spacer />
								<v-btn type="submit" text color="primary">Validate Email</v-btn>
							</v-card-actions>
						</v-card>
					</v-form>
				</div>

				<div v-if="part === 1">
					<v-form @submit.prevent="validateEmail">
						<v-card flat>
							<v-card-title class="headline">
								Email validation
							</v-card-title>

							<v-card-text>
								<v-text-field v-model="token" label="Email token" :rules="[rules.required]" />

								<v-text-field
									v-model.lazy="email"
									label="Enter your Email"
									prepend-icon="mdi-account-circle"
									:rules="[rules.required, rules.email]"
								/>

								<v-text-field
									v-model="password"
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
									v-model="password2"
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
								<v-btn type="submit" text color="primary">Reset password</v-btn>
							</v-card-actions>
						</v-card>
					</v-form>
				</div>
			</v-col>
		</v-row>

		<v-dialog v-model="dialog" max-width="700">
			<v-card>
				<v-card-title class="headline">Email was successfully verified</v-card-title>
				<v-card-text>You can close this window.</v-card-text>

				<v-card-actions>
					<v-spacer />
					<v-btn text to="/">Back to start</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>

		<v-dialog v-model="pw_reset_dialog" max-width="700">
			<v-card>
				<v-card-title class="headline">Password was successfully reset.</v-card-title>
				<v-card-text>You can close this window.</v-card-text>

				<v-card-actions>
					<v-spacer />
					<v-btn text to="/">Back to start</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Mutation, Prop} from "nuxt-property-decorator";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {SentcError} from "~/utils/types";
import {done_register, done_reset_password} from "server_dashboard_wasm";
import zxcvbn from "zxcvbn";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
	async fetch()
	{
		const param = this.$route.query?.token;

		if (!param) {
			return;
		}

		this.token = param;

		if (this.part !== 1) {
			await this.validateEmail();
		}
	},
	computed: {
		score() {
			if (this.password.length < 6) {
				return {
					color: "red",
					value: 0
				};
			}

			const result = zxcvbn(this.password);

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
export default class EmailValidation extends Vue
{
	@Prop({required: true})
	private part: number;

	private token = "";

	private email = "";
	private password = "";
	private password2 = "";

	private showPassword = false;

	private dialog = false;
	private pw_reset_dialog = false;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	private rules = {
		required: (value) => { return !!value || "Required."; },
		email: (value) => {
			const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
			return pattern.test(value) || "Invalid e-mail.";
		}
	};

	private async validateEmail()
	{
		if (!this.token || this.token === "") {
			return;
		}

		switch (this.part) {
			case 0:
				await this.registerValidation();
				this.dialog = true;
				break;
			case 1:
				await this.passwordReset();
				this.pw_reset_dialog = true;
				break;
		}
	}

	private async registerValidation()
	{
		try {
			const jwt = await this.getJwt();

			await done_register(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, jwt, this.token);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}
	}

	private async passwordReset()
	{
		if (this.password === "" || this.email === "") {
			return;
		}

		const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;

		if (!pattern.test(this.email)) {
			this.setMsg("Invalid e-mail");
			return;
		}

		if (this.password !== this.password2) {
			this.setMsg("Passwords are not the same");
			return;
		}

		try {
			await done_reset_password(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, this.token, this.email, this.password);
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