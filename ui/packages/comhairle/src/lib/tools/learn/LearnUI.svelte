<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { getLocale } from '$lib/paraglide/runtime.js';
	import { m } from '$lib/paraglide/messages';
	import type {
		LearnPageEntry,
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
	import type { OnSequenceChange } from '$lib/tools/toolSequence';

	type Props = {
		pages: LearnPageEntry[];
		onSequenceChange?: OnSequenceChange;
		conversation?: LocalizedConversationDto;
		availableDocuments?: ComhairleDocument[];
		hasKnowledgeBaseDocs?: boolean;
	};

	let {
		pages,
		onSequenceChange,
		conversation,
		availableDocuments = [],
		hasKnowledgeBaseDocs = false
	}: Props = $props();

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
		// The editor only saves inline pages. A text_content_id page has nothing to render here yet.
		Array.isArray(currentPage) ? currentPage.filter((p) => p.lang === getLocale()) : []
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

	// Not gated on the document fetch: the article server-renders, and a source-document badge
	// upgrades in place when the fetch lands.
	let showSkeleton = delayedFlag(() => isNavigating, 150);

	// The segment fills as pages are left behind, so a single page stays empty until the step
	// completes, like every other tool.
	let progress = $derived(pages.length > 0 ? currentPageNo / pages.length : undefined);
	let pageCount = $derived(
		pages.length > 1 ? m.page_x_of_y({ x: currentPageNo + 1, y: pages.length }) : undefined
	);

	$effect(() => {
		onSequenceChange?.({
			next: isLastPage ? undefined : nextPage,
			previous: currentPageNo > 0 ? prevPage : undefined,
			progress,
			position: pageCount
		});
	});
</script>

<div class="mx-auto flex grow flex-col">
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
		<h1>Sorry this page is currently not available in this language</h1>
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
