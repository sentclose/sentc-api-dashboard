<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-img :src="p('Sentc.png')" max-width="200" />
		</v-row>

		<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col cols="12" sm="10" md="8" lg="6" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-form @submit.prevent="pLogin">
					<v-card>
						<v-card-title class="heading">Login</v-card-title>

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
								@click:append="showPassword = !showPassword"
							/>

							<v-btn text style="font-size: 0.90em" color="primary" to="/customer/password">Forgot your password?</v-btn>
						</v-card-text>

						<v-card-actions>
							<v-spacer />
							<v-btn color="success" type="submit">Login</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-col>
		</v-row>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {p} from "~/utils/utils";
import {CustomerLoginData, SentcError} from "~/utils/types";
import {login, CustomerDoneLoginOutput} from "server_dashboard_wasm";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {Action, Mutation} from "nuxt-property-decorator";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent},
	layout: "default_non_login"
})
export default class extends Vue
{
	private email = "";
	private password = "";

	private showPassword = false;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("customer/Customer/saveData")
	private saveData: (data: CustomerLoginData)=> Promise<void>;

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

	private pLogin()
	{
		return this.login(this.email, this.password);
	}

	public async login(email: string, password: string, link = true)
	{
		if (email === "" || password === "") {
			return false;
		}

		try {
			const data: CustomerDoneLoginOutput = await login(process.env.NUXT_ENV_BASE_URL, email, password);

			await this.saveData({
				email_status: data.get_email_status(),
				validate_email: data.get_validate_email(),
				email_send: data.get_email_send(),
				email: data.get_email(),
				user_id: data.get_user_id(),
				device_id: data.get_device_id(),
				jwt: data.get_jwt(),
				refresh_token: data.get_refresh_token(),
				name: data.get_name(),
				first_name: data.get_first_name(),
				company: data.get_company()
			});
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}
		}

		if (link) {
			return this.$router.push("/");
		}
	}
}
</script>

<style scoped>

</style>