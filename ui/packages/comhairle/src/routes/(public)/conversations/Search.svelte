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
			runSearch();
		}, 300);
	}

	function runSearch() {
		const url = setSearch(page.url, value);
		goto(url.toString(), { replaceState: true, keepFocus: true, noScroll: true });
	}

	function onSubmit(e: SubmitEvent) {
		e.preventDefault(); // stop the browser reloading the page
		clearTimeout(debounceTimer); // don't run the typing search as well
		runSearch();
		(document.activeElement as HTMLElement | null)?.blur(); // closes the phone keyboard
	}
</script>

<div class="relative w-full">
	<!-- No button: results update as people type, and Enter still submits the form. -->
	<form role="search" class="flex w-full items-center gap-2" onsubmit={onSubmit}>
		<div class="relative grow">
			<Search
				class="text-muted-foreground pointer-events-none absolute top-1/2 left-4 size-5 -translate-y-1/2"
			/>
			<label for="conversations-search" class="sr-only">Search conversations</label>
			<Input
				id="conversations-search"
				type="search"
				placeholder={m.conversations_search_placeholder()}
				aria-label="Search conversations"
				{value}
				oninput={onInput}
				class="bg-card border-border h-10 rounded-full border pr-4 pl-11 text-base shadow-none md:text-base"
			/>
		</div>
	</form>
</div>
