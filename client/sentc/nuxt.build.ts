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
// eslint-disable-next-line @typescript-eslint/no-var-requires
const compressionPlugin = require("compression-webpack-plugin");
// eslint-disable-next-line @typescript-eslint/no-var-requires
const zlib = require("zlib");

export default function() {
	this.extendBuild(({plugins}) => {
		//copy the wasm file to static folder

		// eslint-disable-next-line new-cap
		plugins.push(new copyWebpackPlugin({
			patterns: [
				{
					from: wasmOutDir + "/server_dashboard_wasm_bg.wasm",
					to: path.resolve(__dirname, "static")
				}
			]
		}));

		//compress plugin for wasm

		const test = /\.(wasm)$/;

		// eslint-disable-next-line new-cap
		const gzipCompressionPlugin = new compressionPlugin({
			test,
			algorithm: "gzip",
			compressionOptions: {level: zlib.constants.Z_BEST_COMPRESSION}
		});

		// eslint-disable-next-line new-cap
		const brotliCompressionPlugin = new compressionPlugin({
			test,
			filename: "[path][base].br",
			algorithm: "brotliCompress",
			compressionOptions: {
				params: {
					[zlib.constants.BROTLI_PARAM_QUALITY]:
					zlib.constants.BROTLI_MAX_QUALITY
				}
			}
		});

		plugins.push(gzipCompressionPlugin, brotliCompressionPlugin);
	});
}