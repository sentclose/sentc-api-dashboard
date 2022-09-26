import {Module, Mutation, VuexModule} from "vuex-module-decorators";

@Module({
	stateFactory: true
})
export default class ErrorEvent extends VuexModule
{
	private msg = "";

	private open_bar = false;

	get getMsg() {
		return this.msg;
	}

	get getOpen() {
		return this.open_bar;
	}

	@Mutation
	public setMsg(msg: string)
	{
		this.msg = msg;
		this.open_bar = true;
	}

	@Mutation
	public open()
	{
		this.open_bar = true;
	}

	@Mutation
	public close()
	{
		this.open_bar = false;
	}
}