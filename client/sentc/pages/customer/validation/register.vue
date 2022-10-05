<template>
	<EmailValidation :part="0" />
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import EmailValidation from "~/components/Customer/EmailValidation.vue";

@Component({
	// eslint-disable-next-line @typescript-eslint/naming-convention
	components: {EmailValidation}
})
export default class extends Vue
{

}
</script>

<style scoped>

</style>