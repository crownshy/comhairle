<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import MagnifyingGlass from 'svelte-radix/MagnifyingGlass.svelte';
	import { getSearch } from './utils';

	const props: { url: URL } = $props();

	const action = $derived.by(() => {
		const url = new URL(props.url);
		url.searchParams.set('/search', '');
		return '?' + url.searchParams.toString();
	});
	const value = $derived(getSearch(page.url));
</script>

<form id="search-form" class="space-0 flex max-w-sm items-center" {action} method="POST">
	<Input
		name="search"
		placeholder="search"
		size={16}
		{value}
		class="h-8 rounded-r-none border-r-0"
		on:focusout={() => document.getElementById('search-form')?.submit()}
	/>
	<Button size="sm" type="submit" variant="outline" class="rounded-l-none px-2"
		><MagnifyingGlass class="h-4 w-4" /></Button
	>
</form>
