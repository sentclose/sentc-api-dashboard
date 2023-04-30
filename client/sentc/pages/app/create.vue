<template>
	<AppCreate :create="true" :group_id="undefined" />
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import AppCreate from "~/components/App/AppCreate.vue";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {AppCreate}
})
export default class extends Vue
{

}
</script>

<style scoped>

</style>