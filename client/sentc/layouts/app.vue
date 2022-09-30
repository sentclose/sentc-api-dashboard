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

			<v-btn
				icon
				@click.stop="miniVariant = !miniVariant"
			>
				<v-icon>mdi-{{ `chevron-${miniVariant ? 'right' : 'left'}` }}</v-icon>
			</v-btn>

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
			<v-dialog v-model="deleteDialog" max-width="500">
				<Delete @changeDone="deleteDialog = false" />
			</v-dialog>

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
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {CustomerMenu, Delete},
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

	private deleteDialog = false;

	private items = [
		{
			icon: "mdi-apps",
			title: "All apps",
			to: "/app"
		},
		{
			icon: "mdi-chart-bubble",
			title: "Create app",
			to: "/app/create"
		},
		{
			icon: "mdi-apps",
			title: "App overview",
			to: "/app/" + this.$route.params?.appId
		},
		{
			icon: "mdi-key",
			title: "App jwt keys",
			to: "/app/" + this.$route.params?.appId + "/jwt"
		},
		{
			icon: "mdi-key",
			title: "App access token",
			to: "/app/" + this.$route.params?.appId + "/token"
		},
		{
			icon: "mdi-cog",
			title: "App options",
			to: "/app/" + this.$route.params?.appId + "/options"
		}
	];
}
</script>
