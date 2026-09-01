<script lang="ts">
	/** A number that counts up to `value` once, when it first appears. */
	import { countUpValue } from './countUp';

	let {
		value,
		duration = 1200,
		delay = 0
	}: {
		value: number;
		/** How long the count takes, in ms. */
		duration?: number;
		/** How long to wait before starting, in ms. Used to stagger a row of them. */
		delay?: number;
	} = $props();

	let shown = $state(0);

	$effect(() => {
		const target = value;
		// Counting is the whole animation here rather than decoration on top of one, so
		// reduced motion gets the number itself rather than a shortened count. A hidden tab
		// gets it too: frames are not delivered there, so the alternative is a page that
		// says zero until it is looked at.
		if (document.hidden || window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
			shown = target;
			return;
		}

		let frame = 0;
		let start = 0;
		const tick = (now: number) => {
			if (!start) start = now;
			const elapsed = now - start - delay;
			shown = countUpValue(target, elapsed, duration);
			if (elapsed < duration) frame = requestAnimationFrame(tick);
		};
		frame = requestAnimationFrame(tick);
		return () => cancelAnimationFrame(frame);
	});
</script>

{shown}
