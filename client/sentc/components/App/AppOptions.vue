<template>
	<div v-if="!$fetchState.pending">
		<v-card-text>
			The following options set how the endpoints can be access. <br>
			Secret means this endpoint can only be access with the secret app token. Only your backend should known this token. The api can now be sure that this request comes from your backend. <br>
			Public can be access with the public or secret app token.
		</v-card-text>
		<v-card-actions>
			<v-btn text color="primary" @click="defaultOptions">defaults</v-btn>
			<v-btn text color="primary" @click="laxOptions">lax</v-btn>
			<v-spacer />
			<v-btn text color="error" @click="resetOptions">Reset</v-btn>
		</v-card-actions>

		<div style="overflow-x: auto">
			<div style="width: 1165px">
				<v-simple-table>
					<template #default>
						<thead>
							<tr>
								<th class="text-left">
									Endpoint
								</th>
								<th class="text-left">
									Options
								</th>
								<th class="text-left">
									Description
								</th>
							</tr>
						</thead>
						<tbody>
							<!--User endpoints-->
							<!--User register-->
							<tr>
								<td class="endpoint">
									User register
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_register" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Registration of a user. Use secret if you must send more information's about the user.
								</td>
							</tr>

							<!--User delete-->
							<tr>
								<td class="endpoint">
									User delete
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_delete" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Deletes a user account. All devices are deleted too. <br>
									If you collected more information's about the user, use secret.
								</td>
							</tr>

							<!--User device jwt refresh-->
							<tr>
								<td class="endpoint">
									User device jwt refresh
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_jwt_refresh" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Get a new jwt back when using the refresh token. <br>
									There are multiple strategies: Direct request (then use public) or with cookie and your backend (then use secret).
								</td>
							</tr>

							<!--User prepare login-->
							<tr>
								<td class="endpoint">
									User prepare login
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_prepare_login" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Make the first login request to get the right client salt back. It is recommended to use <b>Public</b>.
								</td>
							</tr>

							<!--User done login-->
							<tr>
								<td class="endpoint">
									User done login
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_done_login" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Make the second login request to get the client keys back. It is recommended to use <b>Public</b> <br>
									If you want to load more information's about the user after login, use the jwt in your own backend.
								</td>
							</tr>

							<!--User identifier exists check-->
							<tr>
								<td class="endpoint">
									User identifier exists check
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_exists" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Checks if the user or device name exists for this app. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--User key update exists check-->
							<tr>
								<td class="endpoint">
									User key update
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_key_update" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Updates the user keys. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!----------------------------------------------------------------------------------------->
							<!--User device endpoints-->

							<!--User identifier update-->
							<tr>
								<td class="endpoint">
									User identifier update
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_update" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Change the user or device identifier. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--User change device password-->
							<tr>
								<td class="endpoint">
									User change device password
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_change_password" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Change the device password. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--User reset device password-->
							<tr>
								<td class="endpoint">
									User reset device password
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_reset_password" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Resets the device password if the keys are known. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--User device register-->
							<tr>
								<td class="endpoint">
									User device register
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_device_register" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Register a new device for the user account. Normally no more information's needed. <br>
									It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--User device delete-->
							<tr>
								<td class="endpoint">
									User device delete
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_device_delete" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Delete a device for the user account. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--User device list-->
							<tr>
								<td class="endpoint">
									User device list
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_device_list" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									List all devices for an account. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!----------------------------------------------------------------------------------------->
							<!--User public data endpoints-->

							<!--User public data-->
							<tr>
								<td class="endpoint">
									User public data
								</td>
								<td class="options">
									<v-radio-group v-model="options.user_public_data" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Fetches the users public and verify key. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!----------------------------------------------------------------------------------------->
							<!--Key management-->

							<!--Key register-->
							<tr>
								<td class="endpoint">
									Key register
								</td>
								<td class="options">
									<v-radio-group v-model="options.key_register" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Creates a new symmetric key, which can be used. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Key fetch-->
							<tr>
								<td class="endpoint">
									Key fetch
								</td>
								<td class="options">
									<v-radio-group v-model="options.key_get" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Fetched a registered symmetric key. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!----------------------------------------------------------------------------------------->
							<!--Group endpoints-->

							<!--Group create-->
							<tr>
								<td class="endpoint">
									Group create
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_create" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Creates a new group. If you must collect more information about a group, like name or topic etc. <br>
									then use secret and register the group from your own backend. <br>
									Otherwise use public.
								</td>
							</tr>

							<!--Group fetch-->
							<tr>
								<td class="endpoint">
									Group fetch
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_get" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Get the group keys. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group user key-->
							<tr>
								<td class="endpoint">
									Group user key fetch
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_user_keys" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Get the user group keys. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group user key update check-->
							<tr>
								<td class="endpoint">
									Group user key update check
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_user_update_check" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Check if there was a key rotation and the user must be updated the group keys. This is used in the client when the group data is cached. <br>
									It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group invite-->
							<tr>
								<td class="endpoint">
									Group invite
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_invite" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									A group member can send invite requests to another non group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group auto invite-->
							<tr>
								<td class="endpoint">
									Group auto invite
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_auto_invite" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									A group member can send invite requests to another non group member and it will automatically accepted. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group accept invite-->
							<tr>
								<td class="endpoint">
									Group accept invite
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_accept_invite" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									The non group member can accept the invite request from a group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group reject invite-->
							<tr>
								<td class="endpoint">
									Group reject invite
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_reject_invite" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									The non group member can reject the invite request from a group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group join req-->
							<tr>
								<td class="endpoint">
									Group join request
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_join_req" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									A non group member can send join requests and a group member can accept or reject it. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group accept join req-->
							<tr>
								<td class="endpoint">
									Group accept join request
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_accept_join_req" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Group member can accept a join req from a non group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group reject join req-->
							<tr>
								<td class="endpoint">
									Group reject join request
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_reject_join_req" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Group member can reject the join req from a non group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group invite stop-->
							<tr>
								<td class="endpoint">
									Group invite stop
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_invite_stop" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Block or allow group invites and join requests for this group. This is called by the group admin. <br>
									It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group key rotation-->
							<tr>
								<td class="endpoint">
									Group key rotation
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_key_rotation" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Started the key rotation for a group to create a new group key. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group user delete-->
							<tr>
								<td class="endpoint">
									Group user delete
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_user_delete" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Delete a group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group leave-->
							<tr>
								<td class="endpoint">
									Group leave
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_leave" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									A group member can leave a group. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group list-->
							<tr>
								<td class="endpoint">
									Group list
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_list" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									A list of all groups where the actual user is member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group change rank-->
							<tr>
								<td class="endpoint">
									Group change rank
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_change_rank" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Change a rank of a group member. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--Group delete-->
							<tr>
								<td class="endpoint">
									Group delete
								</td>
								<td class="options">
									<v-radio-group v-model="options.group_delete" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Delete a group. Use secret when collection more information's about the group when creating groups.
								</td>
							</tr>

							<!----------------------------------------------------------------------------------------->
							<!--file endpoints-->

							<!--File register-->
							<tr>
								<td class="endpoint">
									File register
								</td>
								<td class="options">
									<v-radio-group v-model="options.file_register" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Create a new file in the api. No upload is happened here. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--File fetch-->
							<tr>
								<td class="endpoint">
									File fetch
								</td>
								<td class="options">
									<v-radio-group v-model="options.file_get" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Get the file information's to download the file. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--File part upload-->
							<tr>
								<td class="endpoint">
									File part upload
								</td>
								<td class="options">
									<v-radio-group v-model="options.file_part_upload" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Upload a file part to the sentc api. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!--File part download-->
							<tr>
								<td class="endpoint">
									File part download
								</td>
								<td class="options">
									<v-radio-group v-model="options.file_part_download" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Download a file part from the sentc api. It is recommended to use <b>Public</b>
								</td>
							</tr>

							<!----------------------------------------------------------------------------------------->
							<!--search endpoints-->

							<!--File part download-->
							<tr>
								<td class="endpoint">
									Search
								</td>
								<td class="options">
									<v-radio-group v-model="options.file_part_download" row>
										<v-radio label="Public" :value="1" />
										<v-radio label="Secret" :value="2" />
										<v-radio label="Block" :value="0" />
									</v-radio-group>
								</td>
								<td class="des">
									Search an searchable item. Use public if users should search directly from the client.
									A second request is needed to fetch the data.
								</td>
							</tr>
						</tbody>
					</template>
				</v-simple-table>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import {Prop} from "nuxt-property-decorator";
import {AppOptions as AppOptionsType} from "~/utils/types";
import {app_options_default, app_options_lax} from "server_dashboard_wasm";

@Component({
	fetch() {
		if (this.data) {
			this.options = Object.assign({}, this.data);
			return;
		}
		
		this.options = app_options_default();
	}
})
export default class AppOptions extends Vue
{
	@Prop()
	private data: AppOptionsType;

	//@ts-ignore
	public options: AppOptionsType = {};

	private defaultOptions()
	{
		this.options = app_options_default();
	}

	private laxOptions()
	{
		this.options = app_options_lax();
	}

	private resetOptions()
	{
		this.options = this.data ? Object.assign({}, this.data) : app_options_default();
	}
}
</script>

<style scoped>

.endpoint {
	width: 250px !important;
}

.options {
	width: 360px !important;
}

.des {
	width: 800px !important;
}
</style>