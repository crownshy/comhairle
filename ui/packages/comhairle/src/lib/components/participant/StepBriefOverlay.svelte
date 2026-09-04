<script lang="ts">
	import { X } from 'lucide-svelte';
	import SlideView from './SlideView.svelte';
	import StepBriefBar from './StepBriefBar.svelte';
	import * as m from '$lib/paraglide/messages';
	import { fade } from 'svelte/transition';
	import { haptic } from '$lib/utils/haptics';
	import { prefersReducedMotion } from '$lib/utils/reducedMotion';
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
	let isLast = $derived(index >= slides.length - 1);
	// The hint has nowhere to send the reader but back to the step, so its last page closes.
	let label = $derived(isLast ? m.step_brief_close() : m.pager_next());

	function forward() {
		haptic('light');
		if (isLast) onClose();
		else index = Math.min(slides.length - 1, index + 1);
	}

	let fadeDuration = $derived(prefersReducedMotion() ? 0 : 200);

	function onkeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') onClose();
	}
</script>

<svelte:window {onkeydown} />

<!-- The hint takes the whole screen rather than floating a card over it (ADR-0023), so the
	brief reads the same whether it arrives as the cover or is reopened mid-step. -->
<div
	class="bg-background fixed inset-0 z-50 flex flex-col"
	role="dialog"
	aria-modal="true"
	aria-label={m.step_brief_carousel_label()}
	transition:fade={{ duration: fadeDuration }}
>
	<div
		class="mx-auto flex h-[72px] w-full max-w-5xl shrink-0 items-center justify-end px-5 md:h-20 md:px-6"
	>
		<button
			type="button"
			class="text-foreground -m-2 inline-flex size-10 items-center justify-center p-2"
			aria-label={m.step_brief_close()}
			onclick={onClose}
		>
			<X class="size-6" />
		</button>
	</div>

	<div class="flex min-h-0 flex-1 flex-col items-center justify-center overflow-y-auto px-6 py-8">
		{#key index}
			<SlideView
				{slide}
				{title}
				showTitle={index === 0}
				showMeta={isLast}
				{toolConfig}
				{availableDocuments}
				{conversationId}
			/>
		{/key}
	</div>

	<StepBriefBar {label} onForward={forward} />
</div>
