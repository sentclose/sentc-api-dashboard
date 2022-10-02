<template>
	<div>
		<ErrorEvent />

		<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-img :src="p('Sentc.png')" max-width="200" />
		</v-row>

		<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
			<v-col cols="12" sm="10" md="8" lg="6" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-form @submit.prevent="resetPw">
					<v-card flat>
						<v-card-title class="heading">Reset your password</v-card-title>

						<v-card-text>
							<v-text-field
								v-model="email"
								label="Email"
								prepend-icon="mdi-account-circle"
								:rules="[rules.required, rules.email]"
							/>
						</v-card-text>

						<v-card-actions>
							<v-spacer />
							<v-btn type="submit" text color="primary">Reset password</v-btn>
						</v-card-actions>
					</v-card>
				</v-form>
			</v-col>
		</v-row>

		<v-dialog v-model="dialog" max-width="700">
			<v-card>
				<v-card-title class="headline">We send you an email to reset your password</v-card-title>
				<v-card-text>Please verify your email.</v-card-text>

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
import {p} from "~/utils/utils";
import {Action, Mutation} from "nuxt-property-decorator";
import {CustomerLoginData, SentcError} from "~/utils/types";
import {prepare_reset_password} from "../../../../server_dashboard_wasm/pkg";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {ErrorEvent}
})
export default class extends Vue
{
	//this is called without logged in

	private email = "";

	private rules = {
		required: (value) => { return !!value || "Required."; },
		email: (value) => {
			const pattern = /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
			return pattern.test(value) || "Invalid e-mail.";
		}
	};

	private dialog = false;

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	@Action("customer/Customer/saveData")
	private saveData: (data: CustomerLoginData)=> Promise<void>;

	private p(item: string)
	{
		return p(item);
	}

	private async resetPw()
	{
		//send an email

		try {
			await prepare_reset_password(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN, this.email);

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