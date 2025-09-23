<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Markdown from 'svelte-exmarkdown';
	import * as m from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import type { Page } from '$lib/api/api';
	import { marked } from 'marked';
	import { tick } from 'svelte';

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

	let markdown = $derived(marked.parse(content));
	let articleElement: HTMLElement;

	function nextPage() {
		currentPageNo += 1;
		tick().then(() => {
			articleElement?.scrollIntoView({ behavior: 'smooth' });
		});
	}
</script>

<div class="flex grow flex-col">
	{#if content}
		<article class="prose grow overflow-y-auto" bind:this={articleElement}>
			{@html markdown}
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
