import init from "../../../server_dashboard_wasm/pkg/server_dashboard_wasm_cjs";

export default async function() {
	await init(process.env.NODE_ENV === "production" ? "/dashboard/server_dashboard_wasm_bg.wasm" : "/server_dashboard_wasm_bg.wasm");
}