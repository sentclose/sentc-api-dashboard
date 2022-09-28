<template>
	<v-row justify="center" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
		<v-col sm="12" md="12" lg="12" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}" style="max-width: 1300px">
			<v-row justify="center" align="center" class="mt-3" :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-col cols="12" sm="10" md="8" lg="6">
					<v-card>
						<v-card-title class="headline">Create a new App</v-card-title>
					</v-card>
				</v-col>
			</v-row>

			<v-row :class="{'mx-0': $vuetify.breakpoint.smAndDown, 'px-0': $vuetify.breakpoint.smAndDown}">
				<v-col v-for="(item, i) in appList" :key="i" cols="12" md="6" lg="4">
					<v-card v-if="app(item) !== undefined" exact @click="'/app/'+item">
						<v-card-title>{{ app(item).identifier }}</v-card-title>
						<v-card-text>
							Created: {{ ts(app(item).time) }}
						</v-card-text>
					</v-card>
				</v-col>
			</v-row>
		</v-col>
	</v-row>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Getter} from "nuxt-property-decorator";
import {AppData} from "~/utils/types";
import {getTime} from "~/utils/utils";

@Component
export default class extends Vue
{
	@Getter("app/App/appList")
	private appList: string[];

	@Getter("app/App/app")
	private app: (id: string) => AppData;

	private ts(ts: number)
	{
		return getTime(ts);
	}
}
</script>

<style scoped>

</style>