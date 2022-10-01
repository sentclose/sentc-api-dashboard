<template>
	<div />
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component({
	asyncData({redirect}) {
		return redirect("/");
	}
})
export default class extends Vue
{

}
</script>

<style scoped>

</style>