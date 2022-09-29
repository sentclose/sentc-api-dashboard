<template>
	<div style="max-width: 1200px">
		<h1>Options</h1>

		<v-expansion-panels popout>
			<v-expansion-panel>
				<v-expansion-panel-header expand-icon="mdi-chevron-down">
					<span><v-icon>mdi-cog</v-icon> App options</span>
				</v-expansion-panel-header>
				<v-expansion-panel-content>
					<AppOptions ref="options" />
				</v-expansion-panel-content>
			</v-expansion-panel>
		</v-expansion-panels>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import AppOptions from "~/components/App/AppOptions.vue";
import {Prop} from "nuxt-property-decorator";
@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {AppOptions}
})
export default class AppCreate extends Vue
{
	@Prop({required: true})
	private create: boolean;
}
</script>

<style scoped>

</style>