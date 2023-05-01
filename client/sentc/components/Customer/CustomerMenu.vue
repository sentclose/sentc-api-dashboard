<template>
	<div>
		<v-navigation-drawer
			v-model="model"
			:right="true"
			temporary
			fixed
		>
			<v-list>
				<v-list-item two-line>
					<v-list-item-icon>
						<v-icon>mdi-account</v-icon>
					</v-list-item-icon>
					<v-list-item-content>
						<v-list-item-title>{{ getFirstName }} {{ getName }}</v-list-item-title>
						<v-list-item-subtitle>
							{{ getEmail }}
						</v-list-item-subtitle>
					</v-list-item-content>
				</v-list-item>

				<v-list-item three-line>
					<v-list-item-icon>
						<v-icon>mdi-account</v-icon>
					</v-list-item-icon>
					<v-list-item-content>
						<v-list-item-title>Id:</v-list-item-title>
						<v-list-item-subtitle>
							{{ getUserId }}
						</v-list-item-subtitle>
					</v-list-item-content>
				</v-list-item>

				<v-divider />

				<v-divider />

				<v-list-item to="/customer/billing" router exact>
					<v-list-item-icon>
						<v-icon>mdi-apps</v-icon>
					</v-list-item-icon>
					<v-list-item-content>
						<v-list-item-title v-text="'Billing'" />
					</v-list-item-content>
				</v-list-item>

				<v-list-item to="/customer/update" router exact>
					<v-list-item-icon>
						<v-icon>mdi-account</v-icon>
					</v-list-item-icon>
					<v-list-item-content>
						<v-list-item-title v-text="'Settings'" />
					</v-list-item-content>
				</v-list-item>

				<v-divider />

				<v-divider />

				<v-list-item @click="logOut">
					<v-list-item-icon>
						<v-icon>mdi-logout</v-icon>
					</v-list-item-icon>
					<v-list-item-content>
						<v-list-item-title v-text="'Logout'" />
					</v-list-item-content>
				</v-list-item>
			</v-list>
		</v-navigation-drawer>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter, Prop} from "nuxt-property-decorator";
import {p} from "~/utils/utils";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	watch: {
		value(newVal) {
			this.model = newVal;
		},
		model(newVal) {
			this.$emit("input", this.model);
		}
	},
	created() {
		this.model = this.value;
	}
})
export default class CustomerMenu extends Vue
{
	@Prop()
	private value: boolean;

	private model = false;

	@Getter("customer/Customer/getName")
	private getName: string;

	@Getter("customer/Customer/getFirstName")
	private getFirstName: string;

	@Getter("customer/Customer/getUserId")
	private getUserId: string;

	@Getter("customer/Customer/getEmail")
	private getEmail: string;

	@Action("customer/Customer/logout")
	private logout: () => Promise<void>;

	private async logOut()
	{
		await this.logout();

		location.replace(p(""));
	}
}
</script>

<style scoped>

</style>