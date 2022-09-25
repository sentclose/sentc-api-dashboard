<template>
	<v-app dark>
		<v-navigation-drawer
			v-model="drawer"
			:mini-variant="miniVariant"
			:clipped="clipped"
			fixed
			app
		>
			<v-list>
				<v-list-item
					v-for="(item, i) in items"
					:key="i"
					:to="item.to"
					router
					exact
				>
					<v-list-item-action>
						<v-icon>{{ item.icon }}</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="item.title" />
					</v-list-item-content>
				</v-list-item>
			</v-list>
		</v-navigation-drawer>

		<v-app-bar
			:clipped-left="clipped"
			fixed
			app
			dark
		>
			<v-app-bar-nav-icon @click.stop="drawer = !drawer" />

			<v-btn
				icon
				@click.stop="miniVariant = !miniVariant"
			>
				<v-icon>mdi-{{ `chevron-${miniVariant ? 'right' : 'left'}` }}</v-icon>
			</v-btn>

			<v-toolbar-title v-text="title" />

			<v-spacer />
			<v-btn
				icon
				@click.stop="rightDrawer = !rightDrawer"
			>
				<v-icon>mdi-menu</v-icon>
			</v-btn>
		</v-app-bar>

		<v-main>
			<v-container>
				<Nuxt />
			</v-container>
		</v-main>

		<v-navigation-drawer
			v-model="rightDrawer"
			:right="right"
			temporary
			fixed
		>
			<v-list>
				<v-list-item
					v-for="(item, i) in right_items"
					:key="i"
					:to="item.to"
					router
					exact
				>
					<v-list-item-action>
						<v-icon>{{ item.icon }}</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="item.title" />
					</v-list-item-content>
				</v-list-item>
			</v-list>
		</v-navigation-drawer>

		<v-footer
			:absolute="!fixed"
			app
		>
			<span>&copy; {{ new Date().getFullYear() }}</span>
		</v-footer>
	</v-app>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component({
	name: "DefaultLayout"
	// eslint-disable-next-line @typescript-eslint/naming-convention
	//components: {Delete, ResetPw, ChangePw, Register, Login}
})
export default class extends Vue
{
	private clipped = true;
	private drawer = true;
	private fixed = false;

	private miniVariant = false;
	private right = true;
	private rightDrawer = false;

	private title = "Sentc Dashboard";

	private items = [
		{
			icon: "mdi-apps",
			title: "Welcome",
			to: "/"
		},
		{
			icon: "mdi-chart-bubble",
			title: "Inspire",
			to: "/inspire"
		}
	];

	private right_items = [
		{
			icon: "mdi-apps",
			title: "Sign up",
			to: "/register"
		},
		{
			icon: "mdi-chart-bubble",
			title: "Sign in",
			to: "/login"
		}
	];
}
</script>
