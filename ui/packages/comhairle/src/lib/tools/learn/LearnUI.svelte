<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import * as m from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import type { Page, LocalizedConversationDto } from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { navigating } from '$app/state';
	import LearnTutor from './LearnTutor.svelte';
	import LearnArticleSkeleton from './LearnArticleSkeleton.svelte';

	let {
		pages,
		onDone,
		onNextAction,
		conversation
	}: {
		pages: Array<Page>;
		onDone: () => void;
		onNextAction?: (fn: () => void) => void;
		conversation?: LocalizedConversationDto;
	} = $props();

	let tutorAvailable = $derived(
		!!conversation?.id && !!conversation?.chatBotId && !!conversation?.enableQaChatBot
	);

	let currentPageNo = $state(0);
	let currentPage = $derived(pages[currentPageNo]);
	let currentPageTranslation = $derived(
		(currentPage ?? []).filter((p) => p.lang === getLocale())
	);
	let content = $derived(currentPageTranslation[0]?.content);
	let isLastPage = $derived(currentPageNo === pages.length - 1);
	let pageHeading = $derived(
		(currentPageTranslation[0] as { title?: string } | undefined)?.title ?? ''
	);

	function nextPage() {
		currentPageNo += 1;
		tick().then(() => {
			window.scrollTo(0, 0);
		});
	}

	/** True while SvelteKit is routing to another step / page. */
	let showSkeleton = $derived(!!navigating.to);

	$effect(() => {
		if (onNextAction) {
			onNextAction(isLastPage ? onDone : nextPage);
		}
	});
</script>

<div class="mx-auto flex grow flex-col">
	<!-- Article content: own loading state (route navigation / content not ready) -->
	{#if showSkeleton}
		<LearnArticleSkeleton />
	{:else if content}
		<article class="prose mx-auto w-full grow overflow-y-auto">
			{#key content}
				<ContentRenderer {content} />
			{/key}
		</article>
	{:else}
		<h1>Sorry this page is currently not avaliable in this language</h1>
	{/if}

	{#if tutorAvailable && conversation}
		<div class="mx-auto w-full max-w-[65ch]">
			<LearnTutor
				conversationId={conversation.id}
				pageTitle={pageHeading}
				loading={showSkeleton}
			/>
		</div>
	{/if}

	{#if currentPageNo == pages.length - 1}
		<Button class="mt-10" onclick={onDone} disabled={showSkeleton}>{m.continue_()}</Button>
	{:else}
		<Button class="mt-10" onclick={nextPage} disabled={showSkeleton}>{m.next()}</Button>
	{/if}
</div>
