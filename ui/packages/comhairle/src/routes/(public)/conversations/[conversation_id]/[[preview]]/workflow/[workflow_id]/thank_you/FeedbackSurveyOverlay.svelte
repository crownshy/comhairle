<script lang="ts">
	/**
	 * The survey, taking the whole screen.
	 *
	 * Same shape as the step brief (StepBriefOverlay): a header row with one close button, and a
	 * body that scrolls. The embed sizes itself to each question and asks the page to keep the
	 * active one in view; the body carries the step-scroll attribute so that request lands on
	 * this scroller rather than on the page behind, which is locked anyway.
	 *
	 * Portalled to the body, as PolisAddOpinion is: the page's scroller carries a mask, which
	 * makes a stacking context, so a fixed overlay left inside it would sit under the chrome.
	 */
	import { Portal } from 'bits-ui';
	import { X } from 'lucide-svelte';
	import { fade } from 'svelte/transition';
	import HeyFormEmbed from '$lib/tools/heyform/HeyFormEmbed.svelte';
	import { STEP_SCROLL_ATTRIBUTE } from '$lib/utils/stepScroll';
	import { prefersReducedMotion } from '$lib/utils/reducedMotion';
	import * as m from '$lib/paraglide/messages';

	let {
		userId,
		surveyId,
		serverUrl,
		surveyUrl,
		onDone,
		onClose
	}: {
		userId: string;
		surveyId: string;
		serverUrl: string;
		surveyUrl: string;
		onDone: () => void;
		onClose: () => void;
	} = $props();

	let fadeDuration = $derived(prefersReducedMotion() ? 0 : 200);

	function onkeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') onClose();
	}
</script>

<svelte:window {onkeydown} />

<Portal>
	<div
		class="bg-background fixed inset-0 z-50 flex flex-col"
		role="dialog"
		aria-modal="true"
		aria-label={m.thank_you_feedback_cta()}
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

		<div
			class="flex min-h-0 flex-1 flex-col overflow-y-auto px-5 pb-10 md:px-6"
			{...{ [STEP_SCROLL_ATTRIBUTE]: '' }}
		>
			<HeyFormEmbed
				{surveyId}
				surveyURL={surveyUrl}
				serverURL={serverUrl}
				{userId}
				{onDone}
			/>
		</div>
	</div>
</Portal>
