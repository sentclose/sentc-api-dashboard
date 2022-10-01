<template>
	<div>
		<v-card-text>
			If you want to handle large encrypted files, select your file options: <br>
			Enable the file storage and choose which storage should be used. <br>
			None = disable file handling, own backend = your backend as storage, sentc = sentc api storage.
			<br><br>
			For own backend a storage url (where the files are uploaded to) and an optional auth token for your backend storage to know that this file is uploaded by sentc are needed.
		</v-card-text>

		<v-radio-group v-model="options.file_storage">
			<v-radio label="None" :value="-1" />
			<v-radio label="Sentc api" :value="0" />
			<v-radio label="Own backend" :value="1" />
		</v-radio-group>

		<v-text-field v-model="options.storage_url" :disabled="options.file_storage !== file_storage_own" label="External storage url" />

		<v-text-field v-model="options.auth_token" :disabled="options.file_storage !== file_storage_own" label="External storage auth token" />
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {AppFileOptions as AppFileOptionsType} from "~/utils/types";
import {Mutation, Prop} from "nuxt-property-decorator";

@Component({
	fetch() {
		if (this.data) {
			this.options = Object.assign({}, this.data);
		}
	}
})
export default class AppFileOptions extends Vue
{
	@Prop()
	private data:AppFileOptionsType;

	private file_storage_none = -1;
	private file_storage_sentc = 0;
	private file_storage_own = 1;

	public options: AppFileOptionsType = {
		file_storage: this.file_storage_none,
		auth_token: "",
		storage_url: ""
	};

	@Mutation("event/ErrorEvent/setMsg")
	private setMsg: (msg: string) => void;

	public getOptions()
	{
		if (!this.options.storage_url || this.options.storage_url === "") {
			if (this.options.file_storage === this.file_storage_own) {
				//storage url must be set for own storage
				this.setMsg("Storage url must be set for own backend storage");
				return;
			}

			this.options.storage_url = undefined;
		}

		if (this.options.auth_token === "") {
			this.options.auth_token = undefined;
		}

		return this.options;
	}
}
</script>

<style scoped>

</style>