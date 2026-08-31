<script lang="ts">
	import SlideView from './SlideView.svelte';
	import type { MetaToolConfig } from '$lib/step-brief/slideMeta';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	let {
		slides,
		index,
		title,
		toolConfig,
		availableDocuments = [],
		conversationId
	}: {
		slides: string[];
		index: number;
		title: string;
		toolConfig?: MetaToolConfig | null;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	} = $props();

	let slide = $derived(slides[index] ?? '');
	let isFirst = $derived(index === 0);
	let isLast = $derived(index === Math.max(0, slides.length - 1));
</script>

<!-- The cover is a screen in the flow, so the pager's own arrows move between its slides
	(ADR-0018). No second set of controls here. -->
<section class="flex w-full grow flex-col items-center justify-center px-6 py-8">
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
