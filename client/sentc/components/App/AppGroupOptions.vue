<template>
	<div>
		<v-card-text>
			In the following you can select on which rank group users are allowed to start a group key rotation
			and how many rotation per month per group
		</v-card-text>

		<v-text-field v-model="options.min_rank_key_rotation" type="number" label="Minimal rank do start a key rotation" />
		<v-text-field v-model="options.max_key_rotation_month" type="number" label="Maximum key-rotations per month per group" />
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Mutation, Prop} from "nuxt-property-decorator";
import {AppGroupOptions as AppGroupOptionsType} from "~/utils/types";

@Component({
	fetch() {
		if (this.data) {
			this.options = Object.assign({}, this.data);
		}
	}
})
export default class AppGroupOptions extends Vue
{
	@Prop()
	private data: AppGroupOptionsType;

	public options: AppGroupOptionsType = {
		max_key_rotation_month: 100,
		min_rank_key_rotation: 4
	};

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	public getOptions()
	{
		if (this.options.min_rank_key_rotation < 0) {
			this.setMsg("The rank can't be negative");
			return;
		}

		if (this.options.max_key_rotation_month < 0) {
			this.setMsg("The max rotations can't be negative");
			return;
		}

		return this.options;
	}
}
</script>

<style scoped>

</style>