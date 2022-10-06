<template>
	<div>
		<ErrorEvent />

		<Login ref="login" style="display: none" />

		<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-img :src="p('Sentc.png')" max-width="200" />
		</v-row>

		<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col cols="12" sm="10" md="8" lg="6" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-form @submit.prevent="register">
					<v-card>
						<v-card-title class="headline">
							Create new account
						</v-card-title>
						<v-card-text>
							<v-text-field
								v-model.lazy="email"
								label="Email"
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

							<Captcha ref="captcha" />
						</v-card-text>

						<v-card-actions>
							<v-spacer />

							<v-btn color="success" type="submit">Register</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-col>
		</v-row>

		<v-dialog v-model="dialog" max-width="700">
			<v-card>
				<v-card-title class="headline">Registration was successful</v-card-title>
				<v-card-text>We send an Email to your address. Please verify your Email under this link: <nuxt-link to="/customer/validation/register">validation</nuxt-link>, or just click on the link in the Email.</v-card-text>

				<v-card-text>
					<p>
						Thank you for registration.
					</p>
				</v-card-text>

				<v-card-actions>
					<v-spacer />
					<v-btn text to="/" color="primary">Go to start</v-btn>
				</v-card-actions>
			</v-card>
		</v-dialog>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import zxcvbn from "zxcvbn";
import {register} from "server_dashboard_wasm";
import {SentcError} from "~/utils/types";
import {p} from "~/utils/utils";
import Login from "~/pages/login.vue";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {Mutation} from "nuxt-property-decorator";
import Captcha from "~/components/Customer/Captcha.vue";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {Captcha, Login, ErrorEvent},
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
export default class extends Vue
{
	private email = "";
	private password = "";
	private password2 = "";

	private showPassword = false;

	private dialog = false;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	private rules = {
		required: (value) => { return !!value || "Required."; },
		email: (value) => {
			const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
			return pattern.test(value) || "Invalid e-mail.";
		}
	};

	private p(item: string)
	{
		return p(item);
	}

	private async register()
	{
		if (this.email === "" || this.password === "") {
			return false;
		}

		const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;

		if (!pattern.test(this.email)) {
			this.setMsg("Invalid e-mail");
			return false;
		}

		if (this.password !== this.password2) {
			this.setMsg("Passwords are not the same");
			return false;
		}

		try {
			//@ts-ignore
			const captcha: false | string[] = this.$refs.captcha.getSolution();

			if (!captcha) {
				return;
			}

			await register(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, this.email, this.password, captcha[0], captcha[1]);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);

				if (err.error_message === "Captcha is wrong") {
					//load a new captcha
					//@ts-ignore
					await this.$refs.captcha.getCaptcha();
				}

				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}

			return;
		}

		this.dialog = true;

		//@ts-ignore
		return this.$refs.login.login(this.email, this.password, false);
	}
}
</script>

<style scoped>

</style>