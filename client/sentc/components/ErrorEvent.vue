<template>
	<div v-if="msg && msg !== ''">
		<v-snackbar
			v-model="open"
			transition="slide-x-reverse-transition"
			right
			top
			color="error"
			:timeout="-1"
		>
			{{ msg }}

			<template #action="{attrs}">
				<v-btn icon v-bind="attrs" @click="close_event"><v-icon large>mdi-close</v-icon></v-btn>
			</template>
		</v-snackbar>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Getter, Mutation} from "nuxt-property-decorator";

@Component
export default class ErrorEvent extends Vue
{
	@Getter("event/ErrorEvent/getMsg")
	private msg: string;

	@Getter("event/ErrorEvent/getOpen")
	private open: boolean;

	@Mutation("event/ErrorEvent/close")
	private close_event: () => void;
}
</script>

<style scoped>

</style>