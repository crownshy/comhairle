<script lang="ts">
	import { i18n } from '$lib/i18n';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
	import '../app.css';
	import NavBar from "$lib/components/NavBar.svelte"
	let { children } = $props();
</script>

<ParaglideJS {i18n}>
	<NavBar />
	{@render children()}
</ParaglideJS>
