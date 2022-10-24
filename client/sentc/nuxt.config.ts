import colors from "vuetify/es5/util/colors";
import {NuxtConfig} from "@nuxt/types";
// eslint-disable-next-line @typescript-eslint/no-var-requires
const path = require("path");

const config: NuxtConfig = {
	// Disable server-side rendering: https://go.nuxtjs.dev/ssr-mode
	ssr: false,

	// Target: https://go.nuxtjs.dev/config-target
	target: "static",

	generate: {
		dir: path.resolve(__dirname, "../../dist")
	},

	// Global page headers: https://go.nuxtjs.dev/config-head
	head: {
		titleTemplate: "%s - sentc",
		title: "sentc",
		htmlAttrs: {
			lang: "en"
		},
		meta: [
			{charset: "utf-8"},
			{name: "viewport", content: "width=device-width, initial-scale=1"},
			{hid: "description", name: "description", content: ""},
			{name: "format-detection", content: "telephone=no"}
		],
		link: [
			{rel: "icon", type: "image/x-icon", href: process.env.NODE_ENV === "production" ? "/dashboard/favicon.ico" : "/favicon.ico"}
		]
	},

	// Global CSS: https://go.nuxtjs.dev/config-css
	css: [
		"@mdi/font/css/materialdesignicons.css"
	],

	// Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
	plugins: [
		"~/plugins/axios-accessor",
		{src: "~/plugins/initWasm.ts", mode: "client"}
	],

	// Auto import components: https://go.nuxtjs.dev/config-components
	components: true,

	// Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
	buildModules: [
		// https://go.nuxtjs.dev/typescript
		"@nuxt/typescript-build",
		// https://go.nuxtjs.dev/vuetify
		"@nuxtjs/vuetify",
		"~/nuxt.build.ts",
		"@nuxtjs/google-fonts"
	],

	// Modules: https://go.nuxtjs.dev/config-modules
	modules: [
		// https://go.nuxtjs.dev/axios
		"@nuxtjs/axios"
	],

	router: {
		base: process.env.NODE_ENV === "production" ? "/dashboard/" : "/"
	},

	// Axios module configuration: https://go.nuxtjs.dev/config-axios
	axios: {
		// Workaround to avoid enforcing hard-coded localhost:3000: https://github.com/nuxt-community/axios-module/issues/308
		baseURL: "/"
	},

	googleFonts: {
		families: {
			// eslint-disable-next-line @typescript-eslint/naming-convention
			Roboto: {
				wght: [100, 300, 400, 500, 700, 900]
			}
		},
		download: true,
		base64: true,
		inject: true
	},

	// Vuetify module configuration: https://go.nuxtjs.dev/config-vuetify
	vuetify: {
		defaultAssets: false,
		//customVariables: ["~/assets/variables.scss"],
		theme: {
			//dark: true,
			themes: {
				light: {
					primary_dark: colors.blue.darken2
				},
				dark: {
					primary: colors.blue.darken2,
					accent: colors.grey.darken3,
					secondary: colors.amber.darken3,
					info: colors.teal.lighten1,
					warning: colors.amber.base,
					error: colors.deepOrange.accent4,
					success: colors.green.accent3
				}
			}
		}
	},

	// Build Configuration: https://go.nuxtjs.dev/config-build
	build: {
		postcss: false,

		extend(config) {
			// @ts-ignore
			config.resolve.mainFields = ["main"];
		}
	}
};

export default config;