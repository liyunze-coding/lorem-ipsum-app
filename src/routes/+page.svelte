<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { initializeStores, Toast } from "@skeletonlabs/skeleton";
	import { getToastStore } from "@skeletonlabs/skeleton";
	import type { ToastSettings } from "@skeletonlabs/skeleton";

	initializeStores();

	const toastStore = getToastStore();

	const t: ToastSettings = {
		message: "Copied successfully ✨",
		timeout: 2000,
	};

	let outputText: string = $state(
		`Lorem ipsum nec odio eleifend aliquam sagittis erat mollis tellus etiam, duis iaculis nisl a malesuada lorem nunc tellus fringilla nostra condimentum, dictumst mollis iaculis sollicitudin curabitur odio ultricies laoreet varius. Ultrices gravida est porttitor netus per tempus proin orci condimentum suscipit, per hendrerit aptent quisque fusce nisl facilisis cubilia accumsan. Felis adipiscing nunc malesuada maecenas mi vel taciti erat habitant rutrum, quis suscipit morbi ipsum ultrices a ad feugiat.`
	);

	let paragraphs = $state(2);
	let wordCount = $state(0);

	$effect(() => {
		wordCount = outputText.split(" ").length;
	});

	async function generate(e: Event) {
		e.preventDefault();
		let r: string = await invoke("generate_ipsum", {
			paragraphs: paragraphs,
		});

		outputText = r.replaceAll("\n", "\n\n").substring(0, r.length - 2);
	}

	async function copy() {
		navigator.clipboard.writeText(outputText);
		toastStore.trigger(t);
	}
</script>

<main class="container">
	<Toast />
	<h1 class="h1">Lorem Ipsum Generator</h1>
	<p class="text-lg mt-2">
		Lorem Ipsum is a dummy block of text used in publishing and graphic
		design to fill gaps in the page before the actual words are added to the
		finished product.
	</p>
	<p class="text-lg mt-2">Technologies in this app:</p>
	<ul class="list-disc list-inside">
		<li>Tauri (Rust)</li>
		<li>Skeleton UI (TailwindCSS)</li>
		<li>Sveltekit</li>
		<li>Typescript</li>
	</ul>
	<p class="mt-5">
		Made by <a class="anchor" href="https://github.com/liyunze-coding"
			>RythonDev</a
		>
	</p>

	<form class="flex gap-2 mt-5 items-center" onsubmit={generate}>
		<!-- number value -->
		<input
			type="number"
			name="number"
			id="number"
			min="1"
			max="1000"
			bind:value={paragraphs}
		/>

		<div>paragraphs</div>
		<!-- Generate -->
		<input
			type="submit"
			value="Generate"
			class="btn variant-filled-secondary ml-5"
		/>

		<!-- Copy -->
		<input
			type="button"
			aria-label="copy"
			class="btn variant-filled-tertiary"
			value="Copy"
			onclick={copy}
		/>
	</form>
	<div
		class="px-5 py-5 bg-[#141414] border border-gray-500 border-solid mt-5 font-['Times_new_Roman'] whitespace-pre-line"
	>
		<p class="text-xl">{outputText}</p>

		<p class="mt-3">
			{wordCount} words
		</p>
	</div>
</main>

<style>
	.container {
		padding-left: 5rem;
		padding-right: 5rem;
		padding-top: 2.5rem;
		padding-bottom: 2.5rem;
	}

	form input[type="number"] {
		background: #fafafa;
		color: #141414;
	}

	form input {
		padding-left: 1rem;
		padding-right: 1rem;
		padding-top: 0.25rem;
		padding-bottom: 0.25rem;
		border-radius: 0.375rem;
	}
</style>
