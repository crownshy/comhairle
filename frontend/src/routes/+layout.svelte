<script lang="ts">
	import { i18n } from '$lib/i18n';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
	import '../app.css';
	import NavBar from "$lib/components/NavBar.svelte"
	let { children } = $props();
</script>

<ParaglideJS {i18n}>
	<NavBar />
	<div class='max-w-screen px-20 pt-40 mb-16'>
		{@render children()}
	</div>
</ParaglideJS>
