<script lang="ts">
	import { ChevronLeft, ChevronRight, X } from 'lucide-svelte';
	import SlideView from './SlideView.svelte';
	import * as m from '$lib/paraglide/messages';
	import { carouselSwipe } from './carouselSwipe';
	import type { MetaToolConfig } from '$lib/step-brief/slideMeta';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	let {
		slides,
		title,
		toolConfig,
		availableDocuments = [],
		conversationId,
		onClose
	}: {
		slides: string[];
		title: string;
		toolConfig?: MetaToolConfig | null;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
		onClose: () => void;
	} = $props();

	let index = $state(0);
	let slide = $derived(slides[index] ?? '');
	let canPrev = $derived(index > 0);
	let canNext = $derived(index < slides.length - 1);

	function goTo(next: number) {
		index = Math.max(0, Math.min(slides.length - 1, next));
	}

	// The pager behind the scrim belongs to the step, not to this deck, so the deck carries
	// its own arrows inside the card (ADR-0018).
	const swipe = carouselSwipe(
		() => goTo(index - 1),
		() => goTo(index + 1)
	);

	function onkeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') onClose();
		if (event.key === 'ArrowLeft') goTo(index - 1);
		if (event.key === 'ArrowRight') goTo(index + 1);
	}
</script>

<svelte:window {onkeydown} />

<div class="fixed inset-0 z-50 flex items-center justify-center p-6">
	<button
		type="button"
		class="bg-foreground/70 absolute inset-0"
		aria-label={m.step_brief_close()}
		onclick={onClose}
	></button>

	<div
		class="border-border bg-card relative z-10 flex max-h-[min(680px,calc(100%-2rem))] w-full max-w-[345px] flex-col overflow-hidden rounded-[10px] border p-6 shadow-lg md:max-w-lg"
		role="dialog"
		aria-modal="true"
		aria-label={m.step_brief_carousel_label()}
	>
		<button
			type="button"
			class="text-muted-foreground absolute top-4 right-4 z-10 inline-flex size-5 items-center justify-center"
			aria-label={m.step_brief_close()}
			onclick={onClose}
		>
			<X class="size-4" />
		</button>

		<div
			class="flex min-h-0 flex-1 flex-col items-center justify-center overflow-auto pt-6"
			role="group"
			aria-roledescription="carousel"
			onpointerdown={swipe.onpointerdown}
			onpointerup={swipe.onpointerup}
		>
			{#key index}
				<SlideView
					{slide}
					{title}
					showTitle={index === 0}
					showMeta={index === slides.length - 1}
					{toolConfig}
					{availableDocuments}
					{conversationId}
				/>
			{/key}
		</div>

		{#if slides.length > 1}
			<div class="flex shrink-0 items-center justify-between pt-6">
				<button
					type="button"
					class="text-foreground inline-flex size-8 items-center justify-center disabled:opacity-30"
					aria-label={m.pager_back()}
					disabled={!canPrev}
					onclick={() => goTo(index - 1)}
				>
					<ChevronLeft class="size-6" />
				</button>
				<span class="text-muted-foreground text-sm">
					{m.step_brief_slide_position({ current: index + 1, total: slides.length })}
				</span>
				<button
					type="button"
					class="text-foreground inline-flex size-8 items-center justify-center disabled:opacity-30"
					aria-label={m.pager_next()}
					disabled={!canNext}
					onclick={() => goTo(index + 1)}
				>
					<ChevronRight class="size-6" />
				</button>
			</div>
		{/if}
	</div>
</div>
