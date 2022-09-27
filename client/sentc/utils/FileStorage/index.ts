import {StorageFactory} from "~/utils/FileStorage/StorageFactory";
import {StorageInterface} from "~/utils/FileStorage/StorageInterface";

export * from "./StorageFactory";
export * from "./StorageInterface";
export * from "./IndexeddbStorage";
export * from "./MemoryStorage";

export class Storage
{
	private static init_storage = false;

	private static storage: StorageInterface;

	public static async getStore()
	{
		//only init when needed
		if (this.init_storage) {
			//dont init again
			return this.storage;
		}

		this.storage = await StorageFactory.getStorage(({err, warn}) => {
			console.error(err);
			console.warn(warn);
		}, "sentc", "keys");

		this.init_storage = true;

		return this.storage;
	}
}