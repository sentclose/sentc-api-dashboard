<template>
	<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
		<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
			<v-row class="mx-3 mt-3 mb-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<h1 class="display-1">Apps</h1> <v-spacer />
				<v-btn color="primary" to="/app/create">New app</v-btn>
			</v-row>

			<v-row :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" class="mb-5">
				<v-col v-for="(item, i) in appList" :key="i" cols="12" md="6" lg="4">
					<v-hover v-if="app(item) !== undefined" v-slot="{hover}">
						<v-card exact :to="'/app/'+item" :elevation="hover ? 12 : 2">
							<v-card-title>{{ app(item).group_name ? app(item).group_name + "/" : "" }}{{ app(item).identifier ? app(item).identifier : "unnamed" }}</v-card-title>
							<v-card-text>
								Created: {{ ts(app(item).time) }}
							</v-card-text>
						</v-card>
					</v-hover>
				</v-col>
			</v-row>

			<v-row v-if="!appListEnd" align="center" justify="center" class="my-5">
				<v-btn color="primary" text @click="loadMore">Load more</v-btn>
			</v-row>
		</v-col>
	</v-row>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter} from "nuxt-property-decorator";
import {AppData} from "~/utils/types";
import {getTime} from "~/utils/utils";

@Component({
	middleware: ({store, redirect}) => {
		const logged_in = store.getters["customer/Customer/loggedIn"];

		if (logged_in !== 1) {
			return redirect("/login");
		}
	},
	async fetch() {
		if (this.appList.length > 0) {
			return;
		}

		await this.fetchApps();
	}
})
export default class extends Vue
{
	private btn_loading = false;

	@Getter("app/App/appList")
	private appList: string[];

	@Getter("app/App/app")
	private app: (id: string) => AppData;

	@Getter("app/App/appListEnd")
	private appListEnd: boolean;

	@Action("app/App/fetchApps")
	private fetchApps: () => Promise<void>;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	private async loadMore()
	{
		this.btn_loading = true;

		await this.fetchApps();

		this.btn_loading = false;
	}
}
</script>

<style scoped>

</style>