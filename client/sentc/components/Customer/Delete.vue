<template>
	<v-form @submit.prevent="deleteUser">
		<v-card>
			<v-card-title class="heading">
				Delete account
			</v-card-title>

			<v-card-text>
				<v-text-field
					v-model="password"
					label="Password"
					:type="showPassword ? 'text' : 'password'"
					prepend-icon="mdi-lock"
					:append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
					:rules="[rules.required]"
					messages="Please enter your password to delete the account"
					@click:append="showPassword = !showPassword"
				/>
			</v-card-text>

			<v-divider />
			<v-card-actions>
				<v-spacer />
				<v-btn color="error" type="submit">Delete</v-btn>
			</v-card-actions>
		</v-card>
	</v-form>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Getter, Mutation} from "nuxt-property-decorator";
import {delete_customer} from "server_dashboard_wasm";
import {SentcError} from "~/utils/types";

@Component
export default class Delete extends Vue
{
	private password = "";

	private showPassword = false;

	private rules = {
		required: (value) => { return !!value || "Required."; }
	};

	@Getter("customer/Customer/loggedIn")
	private getLogin: number;

	@Getter("customer/Customer/getEmail")
	private getEmail: string;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Mutation("customer/Customer/setLoginStatus")
	private setLoginStatus: (status: number) => void;

	private async deleteUser()
	{
		if (this.getLogin !== 1 || this.getEmail === "") {
			this.setMsg("Not logged in");
			return false;
		}

		try {
			await delete_customer(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, this.getEmail, this.password);
		} catch (e) {
			try {
				const err: SentcError = JSON.parse(e);
				this.setMsg(err.error_message);
			} catch (e) {
				this.setMsg("An undefined error");
			}

			return;
		}

		this.setLoginStatus(0);
		this.$emit("changeDone", true);

		return this.$router.push("/");
	}
}
</script>

<style scoped>

</style>