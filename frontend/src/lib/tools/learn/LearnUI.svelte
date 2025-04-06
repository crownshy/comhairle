<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Markdown from 'svelte-exmarkdown';
	import * as m from '$lib/paraglide/messages';
	import { languageTag } from '$lib/paraglide/runtime.js';
	import type { Page } from '$lib/api/api';

	let {
		pages,
		onDone
	}: {
		pages: Array<Page>;
		onDone: () => void;
	} = $props();

	console.log({ pages });
	let currentPageNo = $state(0);
	let currentPage = $derived(pages[currentPageNo]);
	let currentPageTranslation = $derived(currentPage.filter((p) => p.lang === languageTag()));
	let content = $derived(currentPageTranslation[0]?.content);

	function nextPage() {
		currentPageNo += 1;
	}
</script>

{#if content}
	<article class="prose overflow-y-auto">
		<Markdown md={JSON.parse(content)} />
	</article>
{:else}
	<h1>Sorry this page is currently not avaliable in this language</h1>
{/if}

{#if currentPageNo == pages.length - 1}
	<Button onclick={onDone}>{m.continue_()}</Button>
{:else}
	<Button onclick={nextPage}>{m.next()}</Button>
{/if}
