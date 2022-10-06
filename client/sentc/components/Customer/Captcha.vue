<template>
	<div v-if="png !== '' && !$fetchState.pending">
		<v-row align="center" justify="center">
			<v-col cols="12" sm="12" md="6" style="max-width: 300px">
				<v-btn icon @click="getCaptcha"><v-icon>mdi-refresh</v-icon></v-btn>

				<v-img :src="'data:image/png/;base64,'+png" />

				<v-text-field v-model="solution" label="Your answer" />
			</v-col>
		</v-row>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {captcha_req} from "server_dashboard_wasm";
import {Mutation} from "nuxt-property-decorator";
import {SentcError} from "~/utils/types";

@Component({
	async fetch() {
		await this.getCaptcha();
	}
})
export default class Captcha extends Vue
{
	private png = "";	//base64 encoded

	private captcha_id = "";

	private solution = "";

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	public getSolution()
	{
		if (!this.captcha_id || this.captcha_id === "" || !this.solution || this.solution === "") {
			this.setMsg("Captcha was not fulfilled");
			return false;
		}

		return [this.solution, this.captcha_id];
	}

	public async getCaptcha()
	{
		try {
			const out = await captcha_req(process.env.NUXT_ENV_BASE_URL, process.env.NUXT_ENV_APP_PUBLIC_TOKEN);

			this.png = out.get_png();
			this.captcha_id = out.get_captcha_id();
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