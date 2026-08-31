<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import type {
		Page,
		LocalizedConversationDto,
		ComhairleDocument
	} from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { navigating } from '$app/state';
	import LearningAssistant from '$lib/components/LearningAssistant/LearningAssistant.svelte';
	import LearnArticleSkeleton from './LearnArticleSkeleton.svelte';
	import { delayedFlag } from '$lib/utils/delayedFlag.svelte';
	import { resolveGlossaryFromMetadata } from '$lib/glossary/localizedGlossary';
	import type { OnSequenceChange } from '$lib/step-brief/toolSequence';

	let {
		pages,
		onSequenceChange,
		conversation,
		availableDocuments = [],
		hasKnowledgeBaseDocs = false
	}: {
		pages: Array<Page>;
		onSequenceChange?: OnSequenceChange;
		conversation?: LocalizedConversationDto;
		availableDocuments?: ComhairleDocument[];
		hasKnowledgeBaseDocs?: boolean;
	} = $props();

	// The assistant only answers from parsed knowledge base documents, so it is hidden entirely
	// when the knowledge base is empty. hasKnowledgeBaseDocs is the single source of truth,
	// hoisted to the workflow +layout.ts and shared with the support sidebar.
	let tutorAvailable = $derived(
		!!conversation?.id &&
			!!conversation?.chatBotId &&
			!!conversation?.enableQaChatBot &&
			hasKnowledgeBaseDocs
	);

	let currentPageNo = $state(0);
	let currentPage = $derived(pages[currentPageNo]);
	let currentPageTranslation = $derived(
		(currentPage ?? []).filter((p) => p.lang === getLocale())
	);
	let content = $derived(currentPageTranslation[0]?.content);
	// Glossary is stored on the conversation's metadata jsonb (edited in the admin Configure ->
	// Glossary tab); terms get an auto tooltip in the rendered article, resolved to the
	// participant's current locale (falling back to the conversation's primary locale).
	let glossary = $derived(
		resolveGlossaryFromMetadata(
			conversation?.metadata,
			getLocale(),
			conversation?.primaryLocale ?? 'en'
		)
	);
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

	/** True while SvelteKit is routing to another step. */
	let isNavigating = $derived(!!navigating.to);

	/**
	 * Skeleton only, so a step hop that resolves quickly never renders one and can't flash.
	 * See delayedFlag for the reasoning.
	 *
	 * Deliberately not gated on the document fetch, unlike before: the article server-renders
	 * now, and withholding it for a client-only fetch would blank content that is already on
	 * screen. A source-document badge instead renders its placeholder label and upgrades in
	 * place when the fetch lands, which is a far smaller change than hiding the whole article.
	 */
	let showSkeleton = delayedFlag(() => isNavigating, 150);

	// Learn's pages are the tool-internal sequence the pager traverses before it reaches the
	// step boundary (ADR-0018). Undefined `next` on the last page is what pops the pager out.
	$effect(() => {
		onSequenceChange?.({
			next: isLastPage ? undefined : nextPage,
			prev: currentPageNo > 0 ? prevPage : undefined,
			progress: pages.length > 0 ? (currentPageNo + 1) / pages.length : undefined
		});
	});
</script>

<div class="mx-auto flex grow flex-col">
	<!-- Article content: own loading state (route navigation / content not ready) -->
	{#if showSkeleton.current}
		<LearnArticleSkeleton />
	{:else if content}
		<article class="prose mx-auto w-full grow overflow-y-auto">
			<ContentRenderer
				{content}
				{availableDocuments}
				conversationId={conversation?.id}
				{glossary}
			/>
		</article>
	{:else}
		<h1>Sorry this page is currently not avaliable in this language</h1>
	{/if}

	{#if tutorAvailable && conversation}
		<div class="mx-auto w-full max-w-[65ch]">
			<LearningAssistant
				conversationId={conversation.id}
				pageTitle={pageHeading}
				loading={showSkeleton.current}
			/>
		</div>
	{/if}
</div>
