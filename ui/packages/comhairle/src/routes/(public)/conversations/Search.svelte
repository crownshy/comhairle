<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Input } from '$lib/components/ui/input';
	import { Search } from 'lucide-svelte';
	import { getSearch, setSearch } from './utils';
	import { m } from '$lib/paraglide/messages';

	let value = $state(getSearch(page.url));
	let debounceTimer: ReturnType<typeof setTimeout>;

	$effect(() => {
		value = getSearch(page.url);
	});

	function onInput(e: Event) {
		const input = e.target as HTMLInputElement;
		value = input.value;

		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			const url = setSearch(page.url, value);
			goto(url.toString(), { replaceState: true, keepFocus: true, noScroll: true });
		}, 300);
	}
</script>

<div class="relative w-72">
	<Search
		class="text-muted-foreground pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2"
	/>
	<label for="conversations-search" class="sr-only">Search conversations</label>
	<Input
		id="conversations-search"
		type="search"
		placeholder={m.conversations_search()}
		aria-label="Search conversations"
		{value}
		oninput={onInput}
		class="pl-9"
	/>
</div>
