<script lang="ts">
	import SlideView from './SlideView.svelte';
	import { carouselSwipe } from './carouselSwipe';
	import type { MetaToolConfig } from '$lib/step-brief/slideMeta';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	let {
		slides,
		index,
		title,
		toolConfig,
		availableDocuments = [],
		conversationId,
		onPrev,
		onNext
	}: {
		slides: string[];
		index: number;
		title: string;
		toolConfig?: MetaToolConfig | null;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
		/** Swiping is the pager's back arrow, so it can also leave the cover backwards. */
		onPrev: () => void;
		/** Swiping is the pager's forward arrow, so the last slide starts the step. */
		onNext: () => void;
	} = $props();

	let slide = $derived(slides[index] ?? '');
	let isFirst = $derived(index === 0);
	let isLast = $derived(index === Math.max(0, slides.length - 1));

	const swipe = carouselSwipe(
		() => onPrev(),
		() => onNext()
	);
</script>

<!-- The cover is a screen in the flow, so the pager's own arrows move between its slides
	(ADR-0018). No second set of controls here: swiping is a shortcut to those arrows, not
	a third control. -->
<section
	class="flex w-full grow flex-col items-center justify-center px-6 py-8"
	role="group"
	aria-roledescription="carousel"
	onpointerdown={swipe.onpointerdown}
	onpointerup={swipe.onpointerup}
	onpointercancel={swipe.onpointercancel}
>
	{#key index}
		<SlideView
			{slide}
			{title}
			showTitle={isFirst}
			showMeta={isLast}
			{toolConfig}
			{availableDocuments}
			{conversationId}
		/>
	{/key}
</section>
