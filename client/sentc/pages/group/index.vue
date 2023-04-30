<template>
	<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
		<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
			<v-row class="mx-3 mt-3 mb-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<h1 class="display-1">Groups</h1> <v-spacer />
				<v-btn color="primary" to="/group/create">New group</v-btn>
			</v-row>

			<v-row :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" class="mb-5">
				<v-col v-for="(item, i) in getGroups" :key="i" cols="12" md="6" lg="4">
					<v-hover v-slot="{hover}">
						<v-card v-if="group(item) !== undefined" exact :to="'/group/'+item" :elevation="hover ? 12 : 2">
							<v-card-title>{{ group(item).group_name ? group(item).group_name : "unnamed" }}</v-card-title>
							<v-card-text>
								Created: {{ ts(group(item).time) }}
							</v-card-text>
						</v-card>
					</v-hover>
				</v-col>
			</v-row>

			<v-row v-if="!groupListEnd" align="center" justify="center" class="my-5">
				<v-btn color="primary" text @click="loadMore">Load more</v-btn>
			</v-row>
		</v-col>
	</v-row>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Action, Getter} from "nuxt-property-decorator";
import {GroupData} from "~/utils/types";
import {getTime} from "~/utils/utils";

@Component({
	middleware: ({store, redirect}) => {
		const logged_in = store.getters["customer/Customer/loggedIn"];

		if (logged_in !== 1) {
			return redirect("/login");
		}
	},
	async fetch() {
		if (this.getGroups.length > 0) {
			return;
		}

		await this.fetchGroups();
	}
})
export default class extends Vue
{
	@Getter("group/Group/getGroups")
	private getGroups: string[];

	@Getter("group/Group/groupListEnd")
	private groupListEnd: boolean;

	@Getter("group/Group/group")
	private group: (id: string) => GroupData;

	@Action("group/Group/fetchGroups")
	private fetchGroups: () => Promise<void>;

	private ts(ts: string)
	{
		return getTime(+ts);
	}

	private async loadMore()
	{
		await this.fetchGroups();
	}
}
</script>

<style scoped>

</style>