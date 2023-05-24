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
			dense
		>
			<v-app-bar-nav-icon v-if="getLogin=== 1 && ($vuetify.breakpoint.sm || $vuetify.breakpoint.xs)" @click.stop="drawer = !drawer" />

			<nuxt-link to="/">
				<v-img
					class="mx-2"
					:src="p('Sentc.png')"
					max-width="35"
					max-height="35"
					contain
				/>
			</nuxt-link>

			<nuxt-link to="/" style="text-decoration: none; color: inherit">
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
		</v-app-bar>

		<v-main>
			<v-container fluid :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
					<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
						<Nuxt />
					</v-col>
				</v-row>
			</v-container>
		</v-main>

		<customer-menu v-model="rightDrawer" />

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
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {CustomerMenu},
	middleware: ["getUser"]
})
export default class extends Vue
{
	@Getter("customer/Customer/loggedIn")
	private getLogin: number;

	private clipped = true;
	private drawer = true;
	private fixed = false;

	private miniVariant = false;

	private rightDrawer = false;

	private title = "Sentc Dashboard";

	private p(item: string)
	{
		return p(item);
	}

	private items = [
		{
			icon: "mdi-home",
			title: "App overview",
			to: "/app/" + this.$route.params?.appId
		},
		{
			icon: "mdi-key",
			title: "Jwt keys",
			to: "/app/" + this.$route.params?.appId + "/jwt"
		},
		{
			icon: "mdi-key-variant",
			title: "Access token",
			to: "/app/" + this.$route.params?.appId + "/token"
		},
		{
			icon: "mdi-cog",
			title: "Options",
			to: "/app/" + this.$route.params?.appId + "/options"
		},
		{
			icon: "mdi-file",
			title: "File options",
			to: "/app/" + this.$route.params?.appId + "/file_options"
		},
		{
			icon: "mdi-account-group",
			title: "Group options",
			to: "/app/" + this.$route.params?.appId + "/group_options"
		}
	];
}
</script>
