/**
 * Use a build tool to copy the wasm file to the static dir.
 *
 * When using nuxt config, then the copy plugin is required even for prod env.
 * But in a build module -> this is only required for build times
 */

// eslint-disable-next-line @typescript-eslint/no-var-requires
const path = require("path");
const wasmOutDir = path.resolve(__dirname, "node_modules/server_dashboard_wasm");

// eslint-disable-next-line @typescript-eslint/no-var-requires
const copyWebpackPlugin = require("copy-webpack-plugin");

export default function() {
	this.extendBuild(({plugins}) => {
		// eslint-disable-next-line new-cap
		plugins.push(new copyWebpackPlugin({
			patterns: [
				{
					from: wasmOutDir + "/server_dashboard_wasm_bg.wasm",
					to: path.resolve(__dirname, "static")
				}
			]
		}));
	});
}