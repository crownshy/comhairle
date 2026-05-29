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
		onCanContinueChange,
		conversation,
		isSubmitting = false
	}: {
		pages: Array<Page>;
		onDone: () => void;
		onNextAction?: (fn: () => void) => void;
		onPrevAction?: (fn: (() => void) | undefined) => void;
		onCanContinueChange?: (value: boolean) => void;
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

	let articleEl = $state<HTMLElement | null>(null);
	let isScrollable = $state(false);
	let hasReachedEnd = $state(false);

	const canContinue = $derived(!isScrollable || hasReachedEnd);

	$effect(() => {
		onCanContinueChange?.(canContinue);
	});

	function evaluateScrollState() {
		if (!articleEl) return;
		const scrollable = articleEl.scrollHeight > articleEl.clientHeight + 4;
		isScrollable = scrollable;
		if (!scrollable) {
			hasReachedEnd = true;
			return;
		}
		hasReachedEnd = articleEl.scrollTop + articleEl.clientHeight >= articleEl.scrollHeight - 16;
	}

	function handleArticleScroll() {
		evaluateScrollState();
	}

	$effect(() => {
		// Re-evaluate when page changes or content loads
		currentPageNo;
		content;
		showSkeleton;
		hasReachedEnd = false;
		isScrollable = false;
		tick().then(() => {
			if (articleEl) articleEl.scrollTop = 0;
			evaluateScrollState();
		});
	});

	$effect(() => {
		if (!articleEl) return;
		const observer = new ResizeObserver(() => evaluateScrollState());
		observer.observe(articleEl);
		return () => observer.disconnect();
	});
</script>

<div class="mx-auto flex min-h-0 w-full grow flex-col">
	{#if pages.length > 1}
		<div class="lg:bg-background mb-6 lg:-mx-3 lg:px-3 lg:pt-1 lg:pb-4">
			<div class="mx-auto w-full max-w-[65ch]">
				<p class="text-muted-foreground mb-1.5 text-sm font-medium">
					Page {currentPageNo + 1} of {pages.length}
				</p>
				<Progress
					value={currentPageNo + 1}
					max={pages.length}
					aria-label="Learning progress"
				/>
			</div>
			<!-- fade gradient below indicates scrollable content -->
			<div
				aria-hidden="true"
				class="from-background pointer-events-none absolute inset-x-0 top-full hidden h-6 bg-linear-to-b to-transparent lg:block"
			></div>
		</div>
	{/if}

	<!-- Article content: own loading state (route navigation / content not ready) -->
	{#if showSkeleton}
		<LearnArticleSkeleton />
	{:else if content}
		<div class="relative flex min-h-0 grow flex-col">
			<article
				bind:this={articleEl}
				onscroll={handleArticleScroll}
				class="prose mx-auto min-h-0 w-full grow overflow-y-auto"
			>
				{#key content}
					<ContentRenderer
						{content}
						{availableDocuments}
						conversationId={conversation?.id}
					/>
				{/key}
			</article>
			{#if isScrollable && !hasReachedEnd}
				<div
					aria-hidden="true"
					class="from-background pointer-events-none absolute inset-x-0 bottom-0 h-16 bg-gradient-to-t to-transparent"
				></div>
			{/if}
		</div>
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

	<div class="mt-auto flex flex-col items-center gap-2 pt-6">
		{#if isScrollable && !hasReachedEnd}
			<p class="text-muted-foreground text-xs">Scroll to the end to continue</p>
		{/if}
		{#if currentPageNo == pages.length - 1}
			<Button onclick={onDone} disabled={showSkeleton || isSubmitting || !canContinue}>
				{#if isSubmitting}
					<Spinner class="mr-2 size-4" />
				{/if}
				{m.continue_()}
			</Button>
		{:else}
			<Button onclick={nextPage} disabled={showSkeleton || !canContinue}>{m.next()}</Button>
		{/if}
	</div>
</div>
