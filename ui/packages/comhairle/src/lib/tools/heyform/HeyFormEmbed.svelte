<script lang="ts">
	import { fade } from 'svelte/transition';
	import HeyFormEmbedSkeleton from './HeyFormEmbedSkeleton.svelte';

	type Props = {
		onDone: () => void;
		surveyId: string;
		surveyURL: string;
		serverURL: string;
		userId: string;
		extraSurveyParams?: Record<string, string>;
	};
	let { onDone, surveyId, userId, surveyURL, serverURL, extraSurveyParams }: Props = $props();

	/**
	 * The iframe's `load` fires when the form *document* arrives, but the renderer then boots its
	 * own JS and shows its own spinner on a blank page for a moment. Uncovering at `load` would
	 * hand off skeleton -> white -> their spinner, which is the flicker this grace absorbs.
	 * Mirrors the same trick in HeyFormManage.
	 */
	const RENDERER_BOOT_GRACE_MS = 700;

	// The iframe renders at opacity 0 behind the skeleton until this flips, so it still loads on time.
	let ready = $state(false);
	let firstLoad = $state(true);

	function handleLoad() {
		if (!firstLoad) return;
		firstLoad = false;
		setTimeout(() => (ready = true), RENDERER_BOOT_GRACE_MS);
	}

	/**
	 * Auto-height. The form is a cross-origin iframe, so we can't measure it from here (the browser
	 * blocks reaching into its document). Instead our HeyForm fork measures its own active question
	 * and posts the height out; we listen and size the iframe to it. That gives long / grouped
	 * questions exactly the room they need (no footer overlapping the answers) without leaving a big
	 * empty card on short ones.
	 *
	 * Contract with the fork (see its `sendMessageToParent`):
	 *   { source: 'HEYFORM', eventName: 'FORM_RESIZE', height: <content height in px> }
	 *
	 * `measuredHeight` stays null until that message arrives, so the iframe falls back to the bounded
	 * viewport height in the markup. That keeps this correct against a fork that hasn't shipped the
	 * emit yet: it just behaves like the fixed-height version until the messages start coming.
	 */
	const MIN_FRAME_PX = 440;
	const MAX_FRAME_PX = 2000;

	let measuredHeight = $state<number | null>(null);

	function onFrameMessage(e: MessageEvent) {
		const data = e.data;
		// HeyForm tags every message it posts; ignore anything else on the page (HMR, analytics, ...).
		if (!data || data.source !== 'HEYFORM') return;

		switch (data.eventName) {
			case 'HIDE_EMBED_MODAL':
				setTimeout(() => onDone(), 2000);
				break;
			case 'FORM_RESIZE':
				if (typeof data.height === 'number' && Number.isFinite(data.height)) {
					measuredHeight = Math.min(Math.max(data.height, MIN_FRAME_PX), MAX_FRAME_PX);
				}
				break;
		}
	}

	$effect(() => {
		window.addEventListener('message', onFrameMessage);

		return () => {
			window.removeEventListener('message', onFrameMessage);
		};
	});

	const base_url = $derived.by(() =>
		serverURL.startsWith('https://') ? serverURL : `https://${serverURL}`
	);

	let url = $derived(
		`${base_url}/form/${surveyId}?&amp;id=${surveyId}&amp;type=modal&amp;customUrl=https%3A%2F%2Fforms.crown-shy.com%2Fform%2F&amp;widthType=%25&amp;width=100&amp;heightType=px&amp;height=500&amp;autoResizeHeight=true&polis_id=${userId}&comhairle_user_id=${userId}&hideAfterSubmit=true&autoClose=1`
	);

	let fullUrl = $derived.by(() => {
		if (extraSurveyParams) {
			let params = new URLSearchParams(extraSurveyParams).toString();
			return url + '&' + params;
		}
		return url;
	});
</script>

<!-- The form renderer is a white, self-themed UI inside a cross-origin iframe: we can't restyle its
	internals or match comhairle's light/dark per viewer. So we frame it as a centered white card
	(bg-white is deliberate, matching the form's own paper) rather than a full-bleed slab, so it
	reads as an intentional embedded form on any background, dark mode included. The max-width is
	tunable.

	Height: once the fork reports its content height (measuredHeight, see the FORM_RESIZE handler) we
	size the iframe to exactly that. Until then we fall back to a compact fixed height that matches the
	skeleton, so the pre-emit flash and the loaded form line up. NOTE: this assumes the fork emits
	FORM_RESIZE; a heyform build without that would render every form at this short fixed height, so
	the emit must be deployed alongside this. The height transition smooths the per-question resize. -->
<!-- A single-cell grid rather than absolute positioning: both children claim the same cell, so the
	skeleton inherits the iframe's exact box without restating its height clamp. -->
<div class="grid w-full grid-cols-1 grid-rows-1">
	{#if !ready}
		<div class="z-10 [grid-area:1/1]" out:fade={{ duration: 200 }}>
			<HeyFormEmbedSkeleton />
		</div>
	{/if}
	<div class="mx-auto mt-1 w-full max-w-2xl overflow-hidden rounded-xl [grid-area:1/1]">
		<iframe
			src={fullUrl}
			title="survey"
			onload={handleLoad}
			allow="microphone; camera"
			style={measuredHeight ? `height:${measuredHeight}px` : undefined}
			class="{measuredHeight
				? ''
				: 'min-h-110'} w-full border-none transition-[height,opacity] duration-300 {ready
				? 'opacity-100'
				: 'opacity-0'}"
		></iframe>
	</div>
</div>
