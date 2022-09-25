<template>
	<v-row justify="center" align="center">
		<v-col cols="12" sm="8" md="6">
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
					</v-card-text>

					<v-card-actions>
						<v-spacer />

						<v-btn color="success" type="submit">Register</v-btn>
					</v-card-actions>
				</v-card>
			</v-form>
		</v-col>
	</v-row>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import zxcvbn from "zxcvbn";
import {register} from "../../../server_dashboard_wasm/pkg/server_dashboard_wasm_cjs";
import {SentcError} from "~/utils/types";

@Component({
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

	private rules = {
		required: (value) => { return !!value || "Required."; },
		counter: (value) => { return value.length <= 20 || "Max 20 characters"; },
		email: (value) => {
			const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
			return pattern.test(value) || "Invalid e-mail.";
		}
	};

	private async register()
	{
		if (this.email === "" || this.password === "") {
			return false;
		}

		if (this.password !== this.password2) {
			return false;
		}

		try {
			await register(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, this.email, this.password);
		} catch (e) {
			const err: SentcError = JSON.parse(e);

			console.log(err.error_message);

			console.error(e);
			//TODO handle error
		}

		//TODO log user in
	}
}
</script>

<style scoped>

</style>