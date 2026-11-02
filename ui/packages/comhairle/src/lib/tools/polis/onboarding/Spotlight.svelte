<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { driver, type Driver } from 'driver.js';
	import 'driver.js/dist/driver.css';

	type Props = {
		/** Matches a `data-tour` attribute in the markup. */
		target: string;
		/** Names what is on screen; the spotlight redraws when it changes. Undefined hides it
		 * without counting as a skip. */
		moment: string | undefined;
		/** Escape closes the spotlight, which skips onboarding. */
		onSkip: () => void;
	};

	let { target, moment, onSkip }: Props = $props();

	const STAGE_PADDING_PX = 16;

	let instance: Driver | undefined;
	let closingOurselves = false;
	let pendingFrame = 0;
	// Content loading in (a skeleton becoming the groups) grows the lit area without any
	// event driver.js hears, so its size is watched directly.
	let sizeWatcher: ResizeObserver | undefined;

	function remeasure() {
		if (pendingFrame) return;
		pendingFrame = requestAnimationFrame(() => {
			pendingFrame = 0;
			if (instance?.isActive()) instance.refresh();
		});
	}

	function ensureDriver(): Driver {
		instance ??= driver({
			animate: false,
			// Dark enough that the step chrome behind drops away and only the lit area reads.
			overlayColor: '#000',
			overlayOpacity: 0.85,
			stagePadding: STAGE_PADDING_PX,
			stageRadius: STAGE_PADDING_PX,
			smoothScroll: false,
			// A tap outside does nothing, so a stray thumb does not lose the flow.
			overlayClickBehavior: () => {},
			onDestroyed: () => {
				instance = undefined;
				if (closingOurselves) closingOurselves = false;
				else onSkip();
			}
		});
		return instance;
	}

	// The instruction lives in the lit area itself rather than a driver.js popover, which
	// painted in one place and jumped once the card settled.
	function show() {
		const element = document.querySelector(`[data-tour="${target}"]`);
		if (!element) {
			hide();
			return;
		}
		ensureDriver().highlight({ element, disableActiveInteraction: false });
		sizeWatcher ??= new ResizeObserver(remeasure);
		sizeWatcher.disconnect();
		sizeWatcher.observe(element);
	}

	function hide() {
		sizeWatcher?.disconnect();
		if (!instance) return;
		closingOurselves = true;
		instance.destroy();
	}

	$effect(() => {
		const current = moment;
		untrack(() => (current ? show() : hide()));
	});

	// driver.js only follows window scroll and resize. The participant step scrolls an inner
	// element and cards fly in, so the lit area is remeasured after either. Neither event
	// bubbles, but both reach the document in the capture phase.
	onMount(() => {
		const events = ['scroll', 'animationend', 'transitionend', 'introend'] as const;
		for (const name of events) {
			document.addEventListener(name, remeasure, { capture: true, passive: true });
		}
		window.addEventListener('resize', remeasure);
		return () => {
			for (const name of events) {
				document.removeEventListener(name, remeasure, { capture: true });
			}
			window.removeEventListener('resize', remeasure);
			cancelAnimationFrame(pendingFrame);
			hide();
		};
	});
</script>

<style>
	/* The pager carries onboarding's Next, so it sits above the overlay and stays usable.
	   Unlayered so it beats driver.js's own unlayered rules. */
	:global(.driver-active [data-spotlight-keep]) {
		z-index: 10001;
	}

	:global(.driver-active [data-spotlight-keep]),
	:global(.driver-active [data-spotlight-keep] *) {
		pointer-events: auto;
	}

	/* driver.js's own animation also moves the lit area, so only the dimming fades in. */
	@media (prefers-reduced-motion: no-preference) {
		:global(.driver-overlay) {
			animation: spotlight-dim-in 300ms ease-out;
		}
	}

	@keyframes -global-spotlight-dim-in {
		from {
			opacity: 0;
		}
	}
</style>
