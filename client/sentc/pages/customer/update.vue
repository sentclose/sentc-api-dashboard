<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
				<v-form @submit.prevent="changeEmail">
					<v-text-field v-model="email" label="New E-Mail" :rules="[rules.required, rules.email]" />

					<v-btn type="submit">Change E-Mail</v-btn>
				</v-form>
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
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import ErrorEvent from "~/components/ErrorEvent.vue";
import {Action, Mutation} from "nuxt-property-decorator";
import {update} from "server_dashboard_wasm";
import {SentcError} from "~/utils/types";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent}
})
export default class extends Vue
{
	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Mutation("customer/Customer/setEmail")
	private setEmail: (email: string)=> void;

	@Action("customer/Customer/getJwt")
	private getJwt: () => Promise<string>;

	private email = "";

	private dialog = false;

	private rules = {
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
			await update(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, jwt, this.email);

			this.setEmail(this.email);

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
}
</script>

<style scoped>

</style>