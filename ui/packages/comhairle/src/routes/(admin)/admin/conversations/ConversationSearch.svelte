<script lang="ts">
	import { afterNavigate, goto } from '$app/navigation';
	import { page } from '$app/state';
	import { useDebounce } from 'runed';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { m } from '$lib/paraglide/messages';
	import { searchFromUrl, searchHref } from './conversationFilters';

	let value = $state(searchFromUrl(page.url));

	const updateUrl = useDebounce(() => {
		// searchHref only swaps the query on the current page's URL.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		goto(searchHref(page.url, value), { replaceState: true, keepFocus: true, noScroll: true });
	}, 250);

	// Our own debounced goto already matches the box; other navigations (back, a link) may not.
	afterNavigate(({ type }) => {
		if (type !== 'goto') value = searchFromUrl(page.url);
	});
</script>

<SearchBar
	bind:value
	oninput={updateUrl}
	placeholder={m.conversations_search()}
	aria-label="Search conversations"
	class="bg-card h-10 w-full @4xl:w-80"
/>
