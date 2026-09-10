<script lang="ts">
	import { stepScroller, stepScrollTop } from '$lib/utils/stepScroll';
	import { tick } from 'svelte';
	import { fade } from 'svelte/transition';
	import HeyFormEmbedSkeleton from './HeyFormEmbedSkeleton.svelte';
	import { browser } from '$app/environment';
	import { clampProgress, type OnSequenceChange } from '$lib/step-brief/toolSequence';
	import { readEmbedTheme } from './embedTheme';
	import { themeStore } from '$lib/stores/theme.svelte';

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

	let iframeEl = $state<HTMLIFrameElement>();

	/**
	 * The fork's initial FORM_RESIZE is a one-shot emit. On a hard refresh a cached iframe can boot and
	 * emit it before this component's message listener is attached during hydration, so the height is
	 * missed and the frame stays stuck at its fallback size. To recover, we ping the fork for its
	 * height once the iframe has loaded and keep pinging until one comes back (see the fork's
	 * REQUEST_RESIZE handler). Deterministic, and a no-op the moment a height arrives.
	 */
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
		// '*' rather than base_url: this is a benign height request and the fork gates on
		// `source: 'COMHAIRLE'`, so a redirected survey origin can't silently drop it.
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

	/**
	 * Keeping the form in our colours (see embedTheme). The URL the frame boots with already carries
	 * the palette, so this is only for the palette changing under a form that is already open: the
	 * viewer hits the light / dark toggle halfway through answering. Rebuilding the URL would be the
	 * obvious move and the wrong one, because changing `src` reloads the frame and throws away
	 * everything they have typed. So the colours go over the same postMessage channel as the resize
	 * handshake and the fork repaints in place.
	 *
	 * `framePalette` is what the frame is currently showing, so a toggle back and forth, or a rerun
	 * of the effect for any other reason, does not post anything. It resets on load because a fresh
	 * document is showing whatever its URL asked for.
	 *
	 * `frameListening` is the race this would otherwise lose. Our `load` fires when the document
	 * arrives, but the renderer attaches its message listener when it mounts, some way after that, so
	 * a theme posted at `load` can land before anything is listening and be dropped. The frame's own
	 * messages are the proof it is up: the first one it sends comes from a mounted renderer, and the
	 * REQUEST_RESIZE ping guarantees one arrives even if the mount emit was missed. So we hold the
	 * palette until we hear from it.
	 */
	let framePalette: string | undefined;
	let frameListening = false;

	/** What the palette hangs off. Both are settled before any component runs (see bootPalette). */
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
		// '*' for the same reason as the resize ping: the fork gates on `source: 'COMHAIRLE'`, and a
		// survey origin that redirects would silently swallow a message addressed to base_url.
		iframeEl?.contentWindow?.postMessage(
			{ source: 'COMHAIRLE', eventName: 'SET_THEME', theme },
			'*'
		);
	}

	/**
	 * Auto-height. The form is a cross-origin iframe, so we can't measure it from here (the browser
	 * blocks reaching into its document). Instead our HeyForm fork measures its own active question
	 * and posts the height out; we listen and size the iframe to it. That gives long / grouped
	 * questions exactly the room they need (no footer overlapping the answers) without leaving a big
	 * empty card on short ones.
	 *
	 * Contract with the fork (see its `sendMessageToParent`), all tagged `source: 'HEYFORM'`:
	 *   FORM_RESIZE      { height: <px> }                    height the frame needs for this question
	 *   FORM_STEP_CHANGE { index, total, percentage }        a new question became active
	 *   HIDE_EMBED_MODAL {}                                  the form finished
	 * And the messages we send back, tagged `source: 'COMHAIRLE'`:
	 *   REQUEST_RESIZE   {}                                  asks the fork to re-emit FORM_RESIZE now
	 *   SET_THEME        { theme: { <colour>: <hex>, ... } } repaints the form in our palette
	 *
	 * `measuredHeight` stays null until FORM_RESIZE arrives, so the iframe falls back to the bounded
	 * viewport height in the markup. That keeps this correct against a fork that hasn't shipped the
	 * emit yet: it just behaves like the fixed-height version until the messages start coming.
	 */
	const MIN_FRAME_PX = 440;
	/**
	 * Not a layout constraint. The form document is `h-screen overflow-hidden` and, below 800px,
	 * hands any overflow to its own inner scroller, so a frame shorter than the content is precisely
	 * the double-scroll bug: the page scrolls and the form scrolls inside it. We follow the reported
	 * height however tall it gets and let the page do the scrolling. This ceiling only rejects a
	 * nonsense number from the frame.
	 */
	const MAX_FRAME_PX = 20000;

	let measuredHeight = $state<number | null>(null);

	/**
	 * Keeping the active question in view. The iframe sizes itself to each question, so the page (not
	 * the frame) is what scrolls: answer a tall question at the bottom and the next one renders above
	 * the fold, or the page shrinks under you and strands you at the footer. The correction is to pull
	 * the frame's top edge back up to the top of the viewport.
	 *
	 * Scrolling the window to 0 is a different thing and the wrong one: a step description can be many
	 * paragraphs long, so top-of-page routinely leaves the question itself off screen.
	 *
	 * Two rules keep this from fighting the user:
	 *   - FORM_STEP_CHANGE is the only trigger. FORM_RESIZE also covers the mount emit and every
	 *     reflow (a textarea growing, validation, fonts settling), so acting on it hijacks someone who
	 *     is reading or typing further down the page, or scrolling while the form is still a skeleton.
	 *   - We only ever scroll up, and only when the frame's top is already above the viewport. If the
	 *     new question starts on screen there is nothing to correct.
	 */
	const FRAME_TOP_MARGIN_PX = 16;
	/**
	 * The height for the new question arrives in a FORM_RESIZE just after the step change. Aligning
	 * before it applies would scroll against the outgoing question's box and then be clamped when the
	 * document resizes, so we wait for it; this bounds that wait.
	 */
	const ALIGN_AFTER_STEP_CHANGE_MS = 150;

	let alignPending = false;
	let alignTimer: ReturnType<typeof setTimeout> | undefined;

	function alignFrameTop() {
		alignPending = false;
		clearTimeout(alignTimer);
		alignTimer = undefined;
		if (!iframeEl) return;

		const scroller = stepScroller();
		const scrolled = stepScrollTop(scroller);
		const scrollerTop = scroller instanceof Window ? 0 : scroller.getBoundingClientRect().top;

		const frameTop = scrolled + iframeEl.getBoundingClientRect().top - scrollerTop;
		const target = Math.max(0, frameTop - FRAME_TOP_MARGIN_PX);
		if (scrolled <= target) return;

		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		scroller.scrollTo({ top: target, behavior: reduceMotion ? 'auto' : 'smooth' });
	}

	function requestFrameTopAlign() {
		alignPending = true;
		clearTimeout(alignTimer);
		alignTimer = setTimeout(alignFrameTop, ALIGN_AFTER_STEP_CHANGE_MS);
	}

	/**
	 * Progress for the chrome's bar (ADR-0018). The fork reports the active question's position
	 * in its logic-applied field list, thank-you screen included, so `index / total` is how far
	 * through the form the participant is. The form's own answered-questions `percentage` also
	 * rides along in the message but is not used: it moves on typing rather than on paging, and
	 * a form where some questions are optional never reaches 100 on it.
	 *
	 * A fork that predates the payload posts the event with no numbers; the bar then holds at
	 * the handover point exactly as before.
	 */
	function reportProgress(index: unknown, total: unknown) {
		if (typeof index !== 'number' || typeof total !== 'number' || total <= 0) return;
		onSequenceChange?.({ progress: clampProgress(index / total) });
	}

	function onFrameMessage(e: MessageEvent) {
		const data = e.data;
		// HeyForm tags every message it posts; ignore anything else on the page (HMR, analytics, ...).
		if (!data || data.source !== 'HEYFORM') return;

		// Anything from the frame means its renderer has mounted, so a theme will be heard now.
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

	/**
	 * Read once, at setup, and deliberately not reactive: it goes into the iframe's `src`, and a
	 * palette that changed the URL would reload the form under whoever is filling it in. Live changes
	 * go through syncFrameTheme instead.
	 *
	 * Reading this early is safe because app.html sets the `dark` class in a pre-paint script and the
	 * server stamps `data-theme` onto the document, so the palette is settled before any component
	 * runs.
	 */
	const bootPalette = browser ? currentPalette() : {};
	const bootKey = paletteKey();

	$effect(() => {
		// Reading the key here is what makes a mode flip or a theme swap rerun this effect. The
		// colours are taken a frame later rather than now: ThemeProvider writes the `dark` class and
		// `data-theme` in an effect of its own, and this way we never sample the outgoing palette if
		// that effect happens to run after this one.
		const key = paletteKey();
		const frame = requestAnimationFrame(() => syncFrameTheme(key));

		return () => cancelAnimationFrame(frame);
	});

	const base_url = $derived.by(() =>
		serverURL.startsWith('https://') ? serverURL : `https://${serverURL}`
	);

	let url = $derived(
		`${base_url}/form/${surveyId}?&amp;id=${surveyId}&amp;type=modal&amp;customUrl=https%3A%2F%2Fforms.crown-shy.com%2Fform%2F&amp;widthType=%25&amp;width=100&amp;heightType=px&amp;height=500&amp;autoResizeHeight=true&polis_id=${userId}&comhairle_user_id=${userId}&hideAfterSubmit=true&autoClose=1`
	);

	let fullUrl = $derived.by(() => {
		// Caller params last: a step that wants a specific colour outranks the ambient palette.
		const params = new URLSearchParams({ ...bootPalette, ...extraSurveyParams }).toString();
		return params ? url + '&' + params : url;
	});
</script>

<!-- The form is a self-themed UI inside a cross-origin iframe, so we can't reach in and style it.
	Instead we hand it our palette and it themes itself: on the URL for the first paint, over
	postMessage for a change after that (see embedTheme and syncFrameTheme). It follows light / dark
	and the deployment's theme without us touching its internals.

	It is still framed as a centered card rather than a full-bleed slab, so it reads as an embedded
	form rather than as the page. That is why the palette maps the form's background to `--card` and
	not `--background`. The max-width is tunable.

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
