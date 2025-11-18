<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as m from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import type { Page } from '$lib/api/api';
	import { tick } from 'svelte';
	import { Markdown } from 'carta-md';
	import { createCarta } from '$lib/utils/carta';

	let {
		pages,
		onDone
	}: {
		pages: Array<Page>;
		onDone: () => void;
	} = $props();

	let currentPageNo = $state(0);
	let currentPage = $derived(pages[currentPageNo]);
	let currentPageTranslation = $derived(currentPage.filter((p) => p.lang === getLocale()));
	let content = $derived(currentPageTranslation[0]?.content);
	let articleElement: HTMLElement | undefined = $state();

	let carta = createCarta();

	function nextPage() {
		currentPageNo += 1;
		tick().then(() => {
			window.scrollTo(0, 0);
		});
	}
</script>

<div class="mx-auto flex grow flex-col">
	{#if content}
		<article class="prose mx-auto w-full grow overflow-y-auto" bind:this={articleElement}>
			{#key content}
				<Markdown {carta} value={content} />
			{/key}
		</article>
	{:else}
		<h1>Sorry this page is currently not avaliable in this language</h1>
	{/if}

	{#if currentPageNo == pages.length - 1}
		<Button class="mt-10" onclick={onDone}>{m.continue_()}</Button>
	{:else}
		<Button class="mt-10" onclick={nextPage}>{m.next()}</Button>
	{/if}
</div>
