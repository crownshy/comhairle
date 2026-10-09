<script lang="ts">
	import { offsetInStepScroll, scrollStepTo, stepScrollTop } from '$lib/utils/stepScroll';
	import { tick } from 'svelte';
	import { fade } from 'svelte/transition';
	import HeyFormEmbedSkeleton from './HeyFormEmbedSkeleton.svelte';
	import { browser } from '$app/environment';
	import { readEmbedTheme } from './embedTheme';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { getLocale } from '$lib/paraglide/runtime';
	import type { OnSequenceChange } from '$lib/tools/toolSequence';

	type Props = {
		onDone: () => void;
		onSequenceChange?: OnSequenceChange;
		surveyId: string;
		surveyURL: string;
		serverURL: string;
		userId: string;
		extraSurveyParams?: Record<string, string>;
	};
	let { onDone, onSequenceChange, surveyId, userId, serverURL, extraSurveyParams }: Props =
		$props();

	// The renderer shows its own spinner for a moment after `load`, so uncovering the frame at
	// `load` would flicker. Same grace as HeyFormManage.
	const RENDERER_BOOT_GRACE_MS = 700;

	// The iframe renders at opacity 0 behind the skeleton until this flips, so it still loads on time.
	let ready = $state(false);
	let firstLoad = $state(true);

	let iframeEl = $state<HTMLIFrameElement>();

	// The fork's first FORM_RESIZE can fire before our listener exists on a hard refresh, so we
	// ask again until a height arrives (NOTES.md, "Height").
	const RESIZE_PING_INTERVAL_MS = 300;
	const RESIZE_PING_TIMEOUT_MS = 5000;
	let pingTimer: ReturnType<typeof setInterval> | undefined;

	function stopResizePing() {
		clearInterval(pingTimer);
		pingTimer = undefined;
	}

	function requestResizeUntilAnswered() {
		stopResizePing();
		let elapsed = 0;
		// '*' rather than base_url: the fork gates on `source: 'COMHAIRLE'`, and a survey origin
		// that redirects would silently drop a message addressed to base_url.
		const ping = () =>
			iframeEl?.contentWindow?.postMessage(
				{ source: 'COMHAIRLE', eventName: 'REQUEST_RESIZE' },
				'*'
			);
		ping();
		pingTimer = setInterval(() => {
			elapsed += RESIZE_PING_INTERVAL_MS;
			if (measuredHeight !== null || elapsed >= RESIZE_PING_TIMEOUT_MS) {
				if (elapsed >= RESIZE_PING_TIMEOUT_MS) {
					console.warn('Resize timeout reached');
				}
				stopResizePing();
				return;
			}
			ping();
		}, RESIZE_PING_INTERVAL_MS);
	}

	function handleLoad() {
		requestResizeUntilAnswered();
		framePalette = bootKey;
		frameListening = false;
		if (!firstLoad) return;
		firstLoad = false;
		setTimeout(() => (ready = true), RENDERER_BOOT_GRACE_MS);
	}

	// A theme change after boot goes over postMessage, because changing `src` would reload the form
	// and lose the answers. The frame only hears it once its renderer has mounted, and its own first
	// message is the proof of that (NOTES.md, "Theme").
	let framePalette: string | undefined;
	let frameListening = false;

	function paletteKey() {
		return `${themeStore.name}:${themeStore.mode}`;
	}

	function currentPalette() {
		const styles = getComputedStyle(document.documentElement);

		return readEmbedTheme((token) => styles.getPropertyValue(token));
	}

	function syncFrameTheme(key: string) {
		if (!frameListening || key === framePalette) return;

		const theme = currentPalette();

		if (!Object.keys(theme).length) return;

		framePalette = key;
		// '*' for the same reason as the resize ping.
		iframeEl?.contentWindow?.postMessage(
			{ source: 'COMHAIRLE', eventName: 'SET_THEME', theme },
			'*'
		);
	}

	// The fork measures its active question and posts the height; we size the iframe to it. Until
	// the first FORM_RESIZE the markup falls back to a fixed height (NOTES.md, "Height").
	const MIN_FRAME_PX = 440;
	// Only rejects a nonsense number. The frame never scrolls inside itself (`hostScroll=true`), so
	// we follow the reported height however tall it gets.
	const MAX_FRAME_PX = 20000;

	let measuredHeight = $state<number | null>(null);

	// After a form step change the new question can sit above the fold, so we pull the frame's
	// top back into view (NOTES.md, "Keeping the question in view").
	const FRAME_TOP_MARGIN_PX = 16;
	// The new question's height lands in a FORM_RESIZE just after the step change. Aligning before
	// it applies would scroll against the old box, so we wait for it; this bounds the wait.
	const ALIGN_AFTER_STEP_CHANGE_MS = 150;

	let alignPending = false;
	let alignTimer: ReturnType<typeof setTimeout> | undefined;

	function alignFrameTop() {
		alignPending = false;
		clearTimeout(alignTimer);
		alignTimer = undefined;
		if (!iframeEl) return;

		const target = Math.max(0, offsetInStepScroll(iframeEl) - FRAME_TOP_MARGIN_PX);
		if (stepScrollTop() <= target) return;
		scrollStepTo(target, { smooth: true });
	}

	function requestFrameTopAlign() {
		alignPending = true;
		clearTimeout(alignTimer);
		alignTimer = setTimeout(alignFrameTop, ALIGN_AFTER_STEP_CHANGE_MS);
	}

	// Fills the chrome's segment by question position (ADR-0047). The form's own `percentage`
	// is ignored because it moves on typing, not paging. An older fork sends no numbers.
	function reportProgress(index: unknown, total: unknown) {
		if (typeof index !== 'number' || typeof total !== 'number' || total <= 0) return;
		onSequenceChange?.({ progress: Math.min(1, Math.max(0, index / total)) });
	}

	function onFrameMessage(e: MessageEvent) {
		const data = e.data;
		// HeyForm tags every message it posts; ignore anything else on the page.
		if (!data || data.source !== 'HEYFORM') return;

		// Any message from the frame means its renderer has mounted and can hear a theme.
		frameListening = true;
		syncFrameTheme(paletteKey());

		switch (data.eventName) {
			case 'HIDE_EMBED_MODAL':
				setTimeout(() => onDone(), 2000);
				break;
			case 'FORM_RESIZE':
				if (typeof data.height === 'number' && Number.isFinite(data.height)) {
					measuredHeight = Math.min(Math.max(data.height, MIN_FRAME_PX), MAX_FRAME_PX);
					// The new question's box is in the DOM after the flush, so measure then.
					if (alignPending) tick().then(alignFrameTop);
				}
				break;
			case 'FORM_STEP_CHANGE':
				requestFrameTopAlign();
				reportProgress(data.index, data.total);
				break;
		}
	}

	$effect(() => {
		window.addEventListener('message', onFrameMessage);

		return () => {
			window.removeEventListener('message', onFrameMessage);
			stopResizePing();
			clearTimeout(alignTimer);
		};
	});

	// Read once and not reactive: it goes into the iframe `src`, and a changed URL would reload the
	// form under whoever is answering. The palette is settled this early because `dark` and
	// `data-theme` are set before any component runs. Later changes go through syncFrameTheme.
	const bootPalette = browser ? currentPalette() : {};
	const bootKey = paletteKey();

	$effect(() => {
		// Reading the key is what reruns this on a mode flip or theme swap. The colours are read a
		// frame later, after ThemeProvider's own effect has written `dark` and `data-theme`.
		const key = paletteKey();
		const frame = requestAnimationFrame(() => syncFrameTheme(key));

		return () => cancelAnimationFrame(frame);
	});

	const base_url = $derived.by(() =>
		serverURL.startsWith('https://') ? serverURL : `https://${serverURL}`
	);

	let url = $derived(
		`${base_url}/form/${surveyId}?&amp;id=${surveyId}&amp;type=modal&amp;customUrl=https%3A%2F%2Fforms.crown-shy.com%2Fform%2F&amp;widthType=%25&amp;width=100&amp;heightType=px&amp;height=500&amp;autoResizeHeight=true&polis_id=${userId}&comhairle_user_id=${userId}&hideAfterSubmit=true&autoClose=1&hostScroll=true`
	);

	let fullUrl = $derived.by(() => {
		// Caller params last: a step that wants a specific colour outranks the ambient palette.
		const params = new URLSearchParams({
			locale: getLocale(),
			...bootPalette,
			...extraSurveyParams
		}).toString();
		return params ? url + '&' + params : url;
	});
</script>

<!-- The form themes itself from the palette we hand it (see embedTheme) and sits in a centered
	card, which is why its background maps to `--card` rather than `--background`. The height
	follows the fork's FORM_RESIZE; until the first one the frame uses the skeleton's fixed height,
	and a fork without the emit renders every form at that height (NOTES.md). -->
<!-- A single-cell grid rather than absolute positioning: both children claim the same cell, so the
	skeleton inherits the iframe's exact box without restating its height clamp. -->
<div class="grid w-full grid-cols-1 grid-rows-1">
	{#if !ready}
		<div class="z-10 [grid-area:1/1]" out:fade={{ duration: 200 }}>
			<HeyFormEmbedSkeleton />
		</div>
	{/if}
	{#if browser}
		<div class="mx-auto mt-1 w-full max-w-2xl overflow-hidden rounded-xl [grid-area:1/1]">
			<iframe
				bind:this={iframeEl}
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
	{/if}
</div>
