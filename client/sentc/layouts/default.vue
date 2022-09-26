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
			<v-app-bar-nav-icon v-if="getLogin=== 1 && $vuetify.breakpoint.sm" @click.stop="drawer = !drawer" />

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
				<v-list-item to="/" router exact>
					<v-list-item-action>
						<v-icon>mdi-apps</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'All apps'" />
					</v-list-item-content>
				</v-list-item>

				<v-list-item to="/billing" router exact>
					<v-list-item-action>
						<v-icon>mdi-apps</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'Billing'" />
					</v-list-item-content>
				</v-list-item>

				<v-divider />

				<v-list-item @click="deleteDialog = !deleteDialog">
					<v-list-item-action>
						<v-icon>mdi-apps</v-icon>
					</v-list-item-action>
					<v-list-item-content>
						<v-list-item-title v-text="'Delete'" />
					</v-list-item-content>
				</v-list-item>
			</v-list>
		</v-navigation-drawer>

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

@Component({
	name: "DefaultLayout",
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {Delete},
	watch: {
		getLogin(val: number) {
			if (this.$vuetify.breakpoint.sm) {
				return;
			}

			if (val === 1 && this.drawer === false) {
				this.drawer = true;
			} else if (val !== 1) {
				this.drawer = false;
			}
		}
	}
})
export default class extends Vue
{
	@Getter("customer/Customer/loggedIn")
	private getLogin: number;

	private clipped = true;
	private drawer = false;
	private fixed = false;

	private miniVariant = false;
	private right = true;
	private rightDrawer = false;

	private title = "Sentc Dashboard";

	private deleteDialog = false;

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
}
</script>
