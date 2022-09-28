<template>
	<div v-if="!$fetchState.pending">
		<v-simple-table fixed-header>
			<template #default>
				<thead>
					<tr>
						<th class="text-left">
							Endpoint
						</th>
						<th class="text-left">
							Options
						</th>
						<th class="text-left">
							Description
						</th>
					</tr>
				</thead>
				<tbody>
					<!--User register-->
					<tr>
						<td>
							User register
						</td>
						<td>
							<v-radio-group v-model="options.user_register" row>
								<v-radio label="Frontend" :value="1" />
								<v-radio label="Backend" :value="2" />
								<v-radio label="Blocked" :value="0" />
							</v-radio-group>
						</td>
						<td>
							Registration of a user. Use backend if you must send more information's about the user.
						</td>
					</tr>

					<!--User prepare login-->
					<tr>
						<td>
							User prepare login
						</td>
						<td>
							<v-radio-group v-model="options.user_prepare_login" row>
								<v-radio label="Frontend" :value="1" />
								<v-radio label="Backend" :value="2" />
								<v-radio label="Blocked" :value="0" />
							</v-radio-group>
						</td>
						<td>
							Make the first login request to get the right client salt back. It is recommended to use Frontend.
						</td>
					</tr>

					<!--User done login-->
					<tr>
						<td>
							User done login
						</td>
						<td>
							<v-radio-group v-model="options.user_done_login" row>
								<v-radio label="Frontend" :value="1" />
								<v-radio label="Backend" :value="2" />
								<v-radio label="Blocked" :value="0" />
							</v-radio-group>
						</td>
						<td>
							Make the second login request to check the client login and get the client keys back. <br>
							It is recommended to use Frontend. <br>
							If you want to load more information's about the user after login, make a request after login to your backend with the jwt.
						</td>
					</tr>
				</tbody>
			</template>
		</v-simple-table>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Prop} from "nuxt-property-decorator";
import {AppOptions as AppOptionsType} from "~/utils/types";
import {app_options_default, app_options_lax} from "server_dashboard_wasm/server_dashboard_wasm_cjs";

@Component({
	fetch() {
		if (this.data) {
			this.options = this.data;
			return;
		}
		
		this.options = app_options_default();
	}
})
export default class AppOptions extends Vue
{
	@Prop()
	private data: AppOptionsType;

	//@ts-ignore
	public options: AppOptionsType = {};

	private defaultOptions()
	{
		this.options = app_options_default();
	}

	private laxOptions()
	{
		this.options = app_options_lax();
	}

	private resetOptions()
	{
		this.options = this.data ? this.data : app_options_default();
	}

	/*
	TODO
	- update and create options
	 */
}
</script>

<style scoped>

</style>