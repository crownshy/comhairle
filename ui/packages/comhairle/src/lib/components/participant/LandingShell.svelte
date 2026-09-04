<script lang="ts">
	/**
	 * The conversation landing page: step zero over the fold, Before you start below it, and
	 * the call to action fixed across the bottom of both (ADR-0021, ADR-0024).
	 *
	 * Shared by the participant route and by an admin participant view, so the two cannot
	 * drift. Unlike a Step this is a scroll-snap deck rather than a fixed three-row grid, so
	 * it is its own shell and reuses nothing from `StepShell`.
	 */
	import type { Snippet } from 'svelte';
	import type { ComhairleDocument, LocalizedConversationDto } from '@crownshy/api-client/api';
	import StepChrome from './StepChrome.svelte';
	import StepZeroScreen from './StepZeroScreen.svelte';
	import BeforeYouStart from './BeforeYouStart.svelte';
	import type { StepItem } from './stepItems';
	import type { StepPreview } from './stepPreview';
	import type { BeforeYouStartPage } from './beforeYouStart';
	import * as m from '$lib/paraglide/messages';

	let {
		conversation,
		steps,
		pages,
		availableDocuments = [],
		preview = false,
		embedded = false,
		onReadMore,
		page,
		callToAction
	}: {
		conversation: LocalizedConversationDto;
		steps: StepPreview[];
		pages: BeforeYouStartPage[];
		availableDocuments?: ComhairleDocument[];
		/** Whether this is an admin walking a draft, which the chrome marks. */
		preview?: boolean;
		/** Whether this is rendered inside something else. See BeforeYouStart. */
		embedded?: boolean;
		onReadMore?: () => void;
		/**
		 * Show one viewport rather than the whole scrolling page: 0 is step zero, and 1 and up
		 * are the Before you start pages. A participant scrolls the lot, so the route leaves
		 * this off; an admin participant view renders one screen per viewport, which is what a
		 * participant has in front of them at each snap position.
		 */
		page?: number;
		callToAction: Snippet;
	} = $props();

	/**
	 * Before you start is step zero, so it is the current item and every real Step is still
	 * ahead. `isIntro` keeps it out of the number of steps a participant is quoted.
	 */
	let stepItems = $derived<StepItem[]>([
		{
			id: 'landing',
			name: m.landing_before_you_start(),
			status: 'current',
			isIntro: true
		},
		...steps.map((step) => ({ id: step.id, name: step.name, status: 'upcoming' as const }))
	]);
</script>

<!-- Step zero owns the first viewport: chrome, cover, call to action, nothing below the fold
     until you scroll. `min-h` rather than a fixed height because the chrome grows on a narrow
     screen and the cover must be allowed to push past the fold rather than clip. -->
{#if page === undefined || page === 0}
	<div class="flex snap-start flex-col pb-28 {embedded ? 'min-h-full' : 'min-h-[100dvh]'}">
		<StepChrome
			steps={stepItems}
			currentIndex={0}
			label={m.landing_before_you_start()}
			fill={0}
			showSupport={false}
			{preview}
		/>

		<StepZeroScreen {conversation} {steps} {onReadMore} />
	</div>
{/if}

{#if page === undefined || page > 0}
	<BeforeYouStart
		{pages}
		{steps}
		conversationId={conversation.id}
		{availableDocuments}
		{embedded}
		page={page === undefined ? undefined : page - 1}
	/>
{/if}

<!-- Fixed rather than sticky: the call to action has to survive the whole scroll through the
     detail, not just the cover. Both blocks above reserve its height. -->
<div
	class="bg-background/70 border-border/40 fixed inset-x-0 bottom-0 z-30 border-t backdrop-blur-lg"
>
	<div class="mx-auto flex w-full max-w-5xl flex-col gap-2 px-5 pt-3 pb-5 md:px-6">
		{@render callToAction()}
	</div>
</div>
