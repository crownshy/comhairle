<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import type {
		Page,
		LocalizedConversationDto,
		ComhairleDocument
	} from '@crownshy/api-client/api';
	import { tick } from 'svelte';
	import { scrollStepToTop } from '$lib/utils/stepScroll';
	import { navigating } from '$app/state';
	import LearningAssistant from '$lib/components/LearningAssistant/LearningAssistant.svelte';
	import LearnArticleSkeleton from './LearnArticleSkeleton.svelte';
	import { delayedFlag } from '$lib/utils/delayedFlag.svelte';
	import { resolveGlossaryFromMetadata } from '$lib/glossary/localizedGlossary';
	import type { OnSequenceChange } from '$lib/step-brief/toolSequence';

	let {
		pages,
		page = 0,
		onSequenceChange,
		conversation,
		availableDocuments = [],
		hasKnowledgeBaseDocs = false
	}: {
		pages: Array<Page>;
		/**
		 * Which page to show. A participant always starts at the first and walks from there;
		 * an admin participant view follows whichever page is open in the editor beside it.
		 */
		page?: number;
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

	// Writable $derived rather than $effect: it resets when the page is chosen from outside,
	// and is also assigned to directly by next/prev. See AGENTS.md on mirroring state. On the
	// participant route `page` never changes, so this behaves exactly as $state(0) did.
	let currentPageNo = $derived(page);
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
			scrollStepToTop();
		});
	}

	function prevPage() {
		currentPageNo -= 1;
		tick().then(() => {
			scrollStepToTop();
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

<div class="mx-auto flex w-full grow flex-col">
	<!-- A page shorter than the screen sits in the middle of the step rather than hugging the
		chrome with dead space beneath it. Auto margins resolve to zero once the page is taller
		than the screen, so a long article still starts at the top and scrolls normally. The
		scroller is the step's <main>, not this element. -->
	<div class="my-auto flex w-full flex-col py-4">
		<!-- Article content: own loading state (route navigation / content not ready) -->
		{#if showSkeleton.current}
			<LearnArticleSkeleton />
		{:else if content}
			<article class="prose mx-auto w-full">
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
			<div class="mx-auto mt-6 w-full max-w-[65ch]">
				<LearningAssistant
					conversationId={conversation.id}
					pageTitle={pageHeading}
					loading={showSkeleton.current}
				/>
			</div>
		{/if}
	</div>
</div>
