<template>
	<v-app dark>
		<v-app-bar
			:clipped-left="clipped"
			fixed
			app
			dark
			dense
		>
			<nuxt-link to="/">
				<v-img
					class="mx-2"
					:src="p('Sentc.png')"
					max-width="35"
					max-height="35"
					contain
				/>
			</nuxt-link>

			<nuxt-link v-show="$vuetify.breakpoint.mdAndUp" to="/" style="text-decoration: none; color: inherit">
				<v-toolbar-title v-text="title" />
			</nuxt-link>

			<v-spacer />
			<v-btn
				v-if="getLogin=== 1"
				icon
				@click.stop="rightDrawer = !rightDrawer"
			>
				<v-icon>mdi-cog</v-icon>
			</v-btn>

			<v-btn v-if="getLogin !== 1" text to="/login" class="mr-3">
				<v-icon left>mdi-login</v-icon> sign in
			</v-btn>

			<v-btn v-if="getLogin !== 1" color="primary" to="/register">
				<v-icon left>mdi-account-plus</v-icon> sign up
			</v-btn>
		</v-app-bar>

		<v-main>
			<v-container fluid :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-tabs>
					<v-tab to="/">Apps</v-tab>
					<v-tab to="/group">Groups</v-tab>
				</v-tabs>

				<Nuxt />
			</v-container>
		</v-main>

		<CustomerMenu v-model="rightDrawer" />

		<v-footer
			:absolute="!fixed"
			app
		>
			<v-card-text style="font-size: 1em;">
				<span>&copy; {{ new Date().getFullYear() }} - Sentclose</span>

				<a style="text-decoration: none" href="https://sentclose.com/impressum/" target="_blank"> Impressum </a>
				<a style="text-decoration: none" href="https://sentclose.com/datenschutz/" target="_blank"> Privacy </a>
			</v-card-text>
		</v-footer>
	</v-app>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Getter} from "nuxt-property-decorator";
import CustomerMenu from "~/components/Customer/CustomerMenu.vue";
import {p} from "~/utils/utils";

@Component({
	name: "DefaultLayout",
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {CustomerMenu},
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

	private p(item: string)
	{
		return p(item);
	}
}
</script>
