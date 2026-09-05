<script lang="ts">
	/**
	 * Runs a tour: a control on the screen circled and explained, one at a time, then a
	 * dismissal remembered so it does not come back (ADR-0032).
	 *
	 * The spotlight, positioning and keyboard handling are driver.js. What lives here is
	 * everything driver.js has no opinion about: naming targets by `data-tour` rather than by
	 * selector, dropping a beat whose control is not on this screen, our copy and our theme
	 * tokens on its popover, and writing the dismissal down.
	 */
	import { onMount } from 'svelte';
	import { driver, type DriveStep } from 'driver.js';
	import 'driver.js/dist/driver.css';
	import './tour.css';
	import * as m from '$lib/paraglide/messages';
	import { prefersReducedMotion } from '$lib/utils/reducedMotion';
	import { markTourSeen } from './seen';
	import type { Tour } from './types';

	let {
		tour,
		scope,
		onDone
	}: {
		tour: Tour;
		/** What the dismissal is remembered against, usually the conversation's id. */
		scope: string;
		/** Called once, however the tour ended: finished, skipped, or Escape. */
		onDone?: () => void;
	} = $props();

	/** Breathing room between the ring and the control it circles. */
	const PADDING = 10;

	let ended = false;

	function end() {
		// driver.js calls onDestroyed on its way out, including the destroy we run ourselves
		// when this component unmounts, so the parent must not be told twice.
		if (ended) return;
		ended = true;
		markTourSeen(tour.id, scope);
		onDone?.();
	}

	onMount(() => {
		/**
		 * Only the beats whose control is on this screen: a step with no brief has no chip, and
		 * a tour that circles an empty corner is worse than one that says less. A beat that
		 * opens its own target is kept regardless, since it has nothing to find yet.
		 */
		const stops = tour.stops.filter(
			(stop) => stop.before || document.querySelector(`[data-tour="${stop.target}"]`)
		);
		if (stops.length === 0) {
			end();
			return;
		}

		const steps: DriveStep[] = stops.map((stop, index) => ({
			element: `[data-tour="${stop.target}"]`,
			onHighlightStarted: () => stop.before?.(),
			waitForElement: stop.waitMs,
			popover: {
				description: stop.text(),
				side: stop.side,
				align: stop.align,
				// driver.js templates its own "{{current}} of {{total}}", which cannot carry a
				// translated ordering, so the count is resolved through paraglide per beat.
				progressText: m.tour_position({ current: index + 1, total: stops.length })
			}
		}));

		const instance = driver({
			steps,
			showProgress: true,
			showButtons: ['next', 'close'],
			// Config level, not per beat: driver.js swaps Next for Done on the last step by
			// overwriting the step's own `nextBtnText`, so a beat that sets one keeps saying
			// Next to the end.
			nextBtnText: m.tour_next(),
			doneBtnText: m.tour_done(),
			popoverClass: 'comhairle-tour',
			animate: !prefersReducedMotion(),
			// Plain black rather than a themed overlay: one built from the foreground token
			// inverts in dark mode, which is what the hand-rolled version did first.
			overlayColor: '#000',
			overlayOpacity: 0.6,
			stagePadding: PADDING,
			stageRadius: 999,
			smoothScroll: true,
			// The circled control looks reachable and is not. Pressing the Next it has just been
			// shown would navigate to the following step and leave the tour pointing at controls
			// that have moved on without it.
			disableActiveInteraction: true,
			overlayClickBehavior: 'close',
			onPopoverRender: (popover) => {
				// driver.js gives its dismissal an unlabelled × in the corner. The tour is four
				// taps long and the way out should say what it does, so it becomes a word in the
				// footer row beside the count.
				popover.closeButton.textContent = m.tour_skip();
				popover.footer.insertBefore(popover.closeButton, popover.footerButtons);
			},
			onDestroyed: end
		});

		instance.drive();

		return () => instance.destroy();
	});
</script>
