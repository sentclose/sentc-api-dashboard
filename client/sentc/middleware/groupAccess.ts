import {Middleware} from "@nuxt/types";

const groupAccess: Middleware = async ({store, route, redirect}) => {
	const group_id = route.params.groupId;

	let group = store.getters["group/Group/group"](group_id);
	
	if (!group) {
		await store.dispatch("group/Group/fetchGroup", group_id);

		group = store.getters["group/Group/group"](group_id);

		if (!group) {
			return redirect("/");
		}
	}
};

export default groupAccess;