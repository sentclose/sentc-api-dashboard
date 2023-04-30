<template>
	<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
		<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
			<v-row class="mx-3 mt-3 mb-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<h1 class="display-1">Apps in {{ getGroup($route.params.groupId) }}</h1> <v-spacer />
				<v-btn color="primary" :to="`/group/${$route.params.groupId}/create_app`">New app in group</v-btn>
			</v-row>
			
			<v-row :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" class="mb-5">
				<v-col v-for="(item, i) in list.app_ids" :key="i" cols="12" md="6" lg="4">
					<v-hover v-slot="{hover}">
						<v-card v-if="app(item) !== undefined" exact :to="'/app/'+item" :elevation="hover ? 12 : 2">
							<v-card-title>{{ app(item).identifier ? app(item).identifier : "unnamed" }}</v-card-title>
							<v-card-text>
								Created: {{ ts(app(item).time) }}
							</v-card-text>
						</v-card>
					</v-hover>
				</v-col>
			</v-row>

			<v-row v-if="!list.end" align="center" justify="center" class="my-5">
				<v-btn color="primary" text @click="loadMore">Load more</v-btn>
			</v-row>
		</v-col>
	</v-row>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter} from "nuxt-property-decorator";
import {AppData, GroupData} from "~/utils/types";
import {getTime} from "~/utils/utils";
import {AppGroupList} from "~/store/app/App";
import groupAccess from "~/middleware/groupAccess";

@Component({
	middleware: [({store, redirect}) => {
		const logged_in = store.getters["customer/Customer/loggedIn"];

		if (logged_in !== 1) {
			return redirect("/login");
		}
	}, groupAccess],
	async fetch() {
		const group_id = this.$route.params.groupId;

		if (this.appList(group_id).app_ids.length === 0) {
			await this.fetchApps(group_id);
		}

		this.list = this.appList(group_id);
	}
})
export default class extends Vue
{
	private btn_loading = false;

	list: AppGroupList = {app_ids: [], end: false};

	@Getter("group/Group/group")
	private getGroup: (id: string)=> GroupData;

	@Getter("app/App/appGroupList")
	private appList: (group_id: string) => AppGroupList;

	@Getter("app/App/app")
	private app: (id: string) => AppData;

	@Action("app/App/fetchAppsInGroup")
	private fetchApps: (group_id: string) => Promise<void>;

	private ts(ts: number)
	{
		return getTime(ts);
	}

	private async loadMore()
	{
		this.btn_loading = true;

		await this.fetchApps(this.$route.params.groupId);

		this.btn_loading = false;
	}
}
</script>

<style scoped>

</style>