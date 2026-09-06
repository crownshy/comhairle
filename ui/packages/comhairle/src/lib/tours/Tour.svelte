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
	import { driver, type Driver, type DriveStep } from 'driver.js';
	import 'driver.js/dist/driver.css';
	import './tour.css';
	import * as m from '$lib/paraglide/messages';
	import { prefersReducedMotion } from '$lib/utils/reducedMotion';
	import { markTourSeen } from './seen';
	import type { Tour, TourStop } from './types';

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
	/** How often to look again for a control that is still mounting. */
	const POLL_MS = 60;

	let ended = false;

	/**
	 * The ring takes the control's own corner radius rather than one number for the whole
	 * tour, plus the padding so it sits concentric with the control rather than parallel to
	 * it. A single radius has to choose: big enough for a pill chip turns a wide panel into a
	 * lozenge, and small enough for the panel squares off the chips.
	 */
	function stageRadiusFor(element: Element | undefined): number {
		if (!element) return PADDING;
		const { width, height } = element.getBoundingClientRect();
		// `rounded-full` computes to a huge px value, so it is clamped to what the box can
		// actually take.
		const own = Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) || 0;
		return Math.min(own, Math.min(width, height) / 2) + PADDING;
	}

	function end() {
		// driver.js calls onDestroyed on its way out, including the destroy we run ourselves
		// when this component unmounts, so the parent must not be told twice.
		if (ended) return;
		ended = true;
		markTourSeen(tour.id, scope);
		onDone?.();
	}

	onMount(() => {
		let cancelled = false;
		let teardown: (() => void) | undefined;

		start().then((stop) => {
			if (cancelled) stop?.();
			else teardown = stop;
		});

		return () => {
			cancelled = true;
			teardown?.();
		};
	});

	function target(stop: TourStop): HTMLElement | null {
		return document.querySelector<HTMLElement>(`[data-tour="${stop.target}"]`);
	}

	/**
	 * Whether this stop's control is on the screen. A stop that gives itself a `waitMs` is
	 * allowed to still be mounting: a step transition takes about a second to put its tool up,
	 * so a control read for too early is read as absent and loses its beat.
	 */
	function present(stop: TourStop): Promise<boolean> {
		if (target(stop)) return Promise.resolve(true);
		if (!stop.waitMs) return Promise.resolve(false);
		return new Promise((resolve) => {
			const deadline = performance.now() + stop.waitMs!;
			const look = () => {
				if (target(stop)) resolve(true);
				else if (performance.now() >= deadline) resolve(false);
				else setTimeout(look, POLL_MS);
			};
			setTimeout(look, POLL_MS);
		});
	}

	async function start(): Promise<(() => void) | undefined> {
		/**
		 * Only the beats whose control is on this screen: a step with no brief has no chip, a
		 * step that is not a Learn page has no assistant, and a tour that circles an empty
		 * corner is worse than one that says less. A beat that opens its own target is kept
		 * regardless, since it has nothing to find yet.
		 *
		 * Resolved before anything is drawn, because the count the participant reads is of the
		 * beats that survive this.
		 */
		const found = await Promise.all(tour.stops.map(present));
		const stops = tour.stops.filter((stop, index) => stop.before || found[index]);
		if (stops.length === 0) {
			// Not marked seen: nothing was shown, so a tour whose screen has not come up yet is
			// still owed to the participant when it does.
			ended = true;
			onDone?.();
			return undefined;
		}

		let instance: Driver | undefined;

		const steps: DriveStep[] = stops.map((stop, index) => ({
			element: `[data-tour="${stop.target}"]`,
			onHighlightStarted: (element) => {
				stop.before?.();
				// Before the stage is drawn for this beat, so the first frame is already right.
				instance?.setConfig({
					...instance.getConfig(),
					stageRadius: stageRadiusFor(element)
				});
			},
			popover: {
				description: stop.text(),
				side: stop.side,
				align: stop.align,
				// driver.js templates its own "{{current}} of {{total}}", which cannot carry a
				// translated ordering, so the count is resolved through paraglide per beat.
				progressText: m.tour_position({ current: index + 1, total: stops.length })
			}
		}));

		instance = driver({
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
			// Replaced per beat by `stageRadiusFor`; this is only what the first frame uses.
			stageRadius: PADDING,
			// Ours, not driver.js's: it nudges a target just inside the edge, and it measures
			// without waiting for the scroll to land. Centring is also kinder on a target that
			// is taller than half the screen.
			smoothScroll: false,
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
			onHighlighted: (element) => {
				element?.scrollIntoView({
					block: 'center',
					behavior: prefersReducedMotion() ? 'auto' : 'smooth'
				});
			},
			onDestroyed: end
		});

		instance.drive();

		/**
		 * driver.js repositions its stage on window scroll, and a participant step scrolls a
		 * `<main>` inside the shell instead, so the hole would stay where the target used to be.
		 * Scroll events do not bubble but do reach the document in the capture phase, so one
		 * listener covers every scroller on the page, including the one the centring above
		 * starts.
		 */
		let queued = 0;
		const onScroll = () => {
			if (queued) return;
			queued = requestAnimationFrame(() => {
				queued = 0;
				if (instance?.isActive()) instance.refresh();
			});
		};
		document.addEventListener('scroll', onScroll, { capture: true, passive: true });

		return () => {
			document.removeEventListener('scroll', onScroll, { capture: true });
			cancelAnimationFrame(queued);
			instance?.destroy();
		};
	}
</script>
