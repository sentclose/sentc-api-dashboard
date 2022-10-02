<template>
	<div>
		<v-navigation-drawer
			v-model="model"
			:right="true"
			temporary
			fixed
		>
			<v-list>
				<v-list-item to="/customer/billing" router exact>
					<v-list-item-action>
						<v-icon>mdi-apps</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'Billing'" />
					</v-list-item-content>
				</v-list-item>

				<v-list-item to="/customer/update" router exact>
					<v-list-item-action>
						<v-icon>mdi-account</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'Update data'" />
					</v-list-item-content>
				</v-list-item>

				<v-divider />

				<v-list-item @click="deleteDialog = !deleteDialog">
					<v-list-item-action>
						<v-icon>mdi-account-remove</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'Delete'" />
					</v-list-item-content>
				</v-list-item>

				<v-divider />

				<v-list-item @click="logOut">
					<v-list-item-action>
						<v-icon>mdi-logout</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'Logout'" />
					</v-list-item-content>
				</v-list-item>
			</v-list>
		</v-navigation-drawer>

		<v-dialog v-model="deleteDialog" max-width="500">
			<Delete @changeDone="deleteDialog = false" />
		</v-dialog>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Prop} from "nuxt-property-decorator";
import Delete from "~/components/Customer/Delete.vue";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {Delete},
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

	private deleteDialog = false;

	@Action("customer/Customer/logout")
	private logout: () => Promise<void>;

	private async logOut()
	{
		await this.logout();

		location.replace("/");
	}
}
</script>

<style scoped>

</style>