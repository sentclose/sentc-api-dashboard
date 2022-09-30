<template>
	<v-app dark>
		<v-app-bar
			:clipped-left="clipped"
			fixed
			app
			dark
			dense
		>
			<v-toolbar-title v-text="title" />

			<v-spacer />
			<v-btn
				v-if="getLogin=== 1"
				icon
				@click.stop="rightDrawer = !rightDrawer"
			>
				<v-icon>mdi-cog</v-icon>
			</v-btn>
		</v-app-bar>

		<v-main>
			<v-container fluid :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<Nuxt />
			</v-container>
		</v-main>

		<CustomerMenu v-model="rightDrawer" />

		<v-footer
			:absolute="!fixed"
			app
		>
			<span>&copy; {{ new Date().getFullYear() }} - Sentclose</span>
		</v-footer>
	</v-app>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Getter} from "nuxt-property-decorator";
import Delete from "~/components/Customer/Delete.vue";
import CustomerMenu from "~/components/Customer/CustomerMenu.vue";

@Component({
	name: "DefaultLayout",
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {CustomerMenu, Delete},
	middleware: ["getUser"]
})
export default class extends Vue
{
	@Getter("customer/Customer/loggedIn")
	private getLogin: number;

	private clipped = true;
	private fixed = false;

	private rightDrawer = false;

	private title = "Sentc Dashboard";

	private items = [
		{
			icon: "mdi-apps",
			title: "Apps",
			to: "/app"
		},
		{
			icon: "mdi-chart-bubble",
			title: "Create app",
			to: "/app/create"
		}
	];
}
</script>
