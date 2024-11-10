import type { Config } from "tailwindcss";
import { join } from "path";

import { skeleton } from "@skeletonlabs/tw-plugin";

export default {
	darkMode: "class",

	content: [
		"./src/**/*.{html,js,svelte,ts}",
		join(
			require.resolve("@skeletonlabs/skeleton"),
			"../**/*.{html,js,svelte,ts}"
		),
	],

	theme: {
		extend: {},
	},

	plugins: [
		// Append the Skeleton plugin (after other plugins)
		skeleton({
			themes: {
				preset: [{ name: "gold-nouveau", enhancements: true }],
			},
		}),
	],
} as Config;