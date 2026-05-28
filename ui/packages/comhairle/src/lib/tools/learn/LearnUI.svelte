<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { Progress } from '$lib/components/ui/progress';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import * as m from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import type {
		Page,
		LocalizedConversationDto,
		ComhairleDocument
	} from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { navigating } from '$app/state';
	import LearnTutor from './LearnTutor.svelte';
	import LearnArticleSkeleton from './LearnArticleSkeleton.svelte';
	import { apiClient } from '@crownshy/api-client/client';

	let {
		pages,
		onDone,
		onNextAction,
		onPrevAction,
		conversation,
		isSubmitting = false
	}: {
		pages: Array<Page>;
		onDone: () => void;
		onNextAction?: (fn: () => void) => void;
		onPrevAction?: (fn: (() => void) | undefined) => void;
		conversation?: LocalizedConversationDto;
		isSubmitting?: boolean;
	} = $props();

	let tutorAvailable = $derived(
		!!conversation?.id && !!conversation?.chatBotId && !!conversation?.enableQaChatBot
	);

	let availableDocuments = $state<ComhairleDocument[]>([]);
	let documentsLoading = $state(true);

	$effect(() => {
		if (!conversation?.id) {
			availableDocuments = [];
			documentsLoading = false;
			return;
		}
		documentsLoading = true;
		apiClient
			.ListDocuments({ params: { conversation_id: conversation.id } })
			.then((docs) => {
				availableDocuments = docs.filter((d) => d.parse_status === 'DONE');
			})
			.catch(() => {
				availableDocuments = [];
			})
			.finally(() => {
				documentsLoading = false;
			});
	});

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

	function prevPage() {
		currentPageNo -= 1;
		tick().then(() => {
			window.scrollTo(0, 0);
		});
	}

	/** True while SvelteKit is routing OR documents for embeds are still being fetched. */
	let showSkeleton = $derived(!!navigating.to || documentsLoading);

	$effect(() => {
		if (onNextAction) {
			onNextAction(isLastPage ? onDone : nextPage);
		}
	});

	$effect(() => {
		onPrevAction?.(currentPageNo > 0 ? prevPage : undefined);
	});
</script>

<div class="mx-auto flex grow flex-col">
	{#if pages.length > 1}
		<div class="mx-auto mb-6 w-full max-w-[65ch]">
			<p class="text-muted-foreground mb-1.5 text-sm font-medium">
				Page {currentPageNo + 1} of {pages.length}
			</p>
			<Progress value={currentPageNo + 1} max={pages.length} aria-label="Learning progress" />
		</div>
	{/if}

	<!-- Article content: own loading state (route navigation / content not ready) -->
	{#if showSkeleton}
		<LearnArticleSkeleton />
	{:else if content}
		<article class="prose mx-auto w-full grow overflow-y-auto">
			{#key content}
				<ContentRenderer {content} {availableDocuments} conversationId={conversation?.id} />
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
		<Button class="mx-auto mt-10" onclick={onDone} disabled={showSkeleton || isSubmitting}>
			{#if isSubmitting}
				<Spinner class="mr-2 size-4" />
			{/if}
			{m.continue_()}
		</Button>
	{:else}
		<Button class="mx-auto mt-10" onclick={nextPage} disabled={showSkeleton}>{m.next()}</Button>
	{/if}
</div>
