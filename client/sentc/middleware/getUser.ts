import {Middleware} from "@nuxt/types";
import {Storage} from "~/utils/FileStorage";
import {CustomerLoginData, USER_KEY_STORAGE_NAMES} from "~/utils/types";

const get_user: Middleware = async ({store}) => {
	const storage = await Storage.getStore();

	const data: CustomerLoginData = await storage.getItem(USER_KEY_STORAGE_NAMES.userData);

	if (!data) {
		store.commit("customer/Customer/setLoginStatus", 0);
		return;
	}

	store.commit("customer/Customer/setData", data);
	store.commit("customer/Customer/setLoginStatus", 1);
};

export default get_user;