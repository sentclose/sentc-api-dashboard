<template>
	<AppCreate :create="true" :group_id="$route.params.groupId" />
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import AppCreate from "~/components/App/AppCreate.vue";
import groupAccess from "~/middleware/groupAccess";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {AppCreate},
	middleware: [({store, redirect}) => {
		const logged_in = store.getters["customer/Customer/loggedIn"];

		if (logged_in !== 1) {
			return redirect("/login");
		}
	}, groupAccess]
})
export default class extends Vue
{

}
</script>

<style scoped>

</style>