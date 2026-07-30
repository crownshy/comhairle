<script lang="ts">
	/**
	 * DivergingProgress
	 *
	 * A progress/score bar that diverges from a zero point rather than
	 * growing from the left edge of the track. Useful for values that can
	 * be negative or positive (e.g. an "alignment score" from -10 to +10),
	 * where a standard 0%-100% progress bar would either clip or hide
	 * negative values entirely.
	 *
	 * The bar always renders a visible reference line at the position of
	 * zero within the given [min, max] domain, then draws a filled segment
	 * from zero out to the current value — extending right for positive
	 * values, left for negative ones. Negative values are rendered in a
	 * different color (`bg-destructive`) so they're distinguishable from
	 * positive values at a glance, independent of bar length.
	 */
	import { cn } from '$lib/utils';

	type Props = {
		value: number;
		min?: number;
		max?: number;
		class?: string;
	};

	let { value, min = -10, max = 10, class: className }: Props = $props();

	let range = $derived(max - min);
	/**
	 * The horizontal position (as a % of track width) where `value === 0`
	 * falls within the [min, max] domain. This is where the reference line
	 * is drawn, and it's also the anchor point that the filled segment
	 * grows from in either direction.
	 */
	let zeroPercentage = $derived(((0 - min) / range) * 100);
	/**
	 * The horizontal position (as a % of track width) corresponding to the
	 * current `value`, mapped from the [min, max] domain onto a [0, 100]
	 * range. This is the "leading edge" of the filled segment.
	 */
	let valuePercentage = $derived(((value - min) / range) * 100);
	/**
	 * The left edge (as a % of track width) of the filled segment.
	 * Since the segment can extend either left (negative values) or right
	 * (positive values) from zero, this is whichever of zeroPct/valuePct
	 * is smaller — i.e. the segment always spans from zero to the value,
	 * regardless of sign.
	 */
	let left = $derived(Math.min(zeroPercentage, valuePercentage));
	/**
	 * The width (as a % of track width) of the filled segment, i.e. the
	 * distance between the zero point and the current value's position.
	 */
	let width = $derived(Math.abs(valuePercentage - zeroPercentage));
	let isNegative = $derived(value < 0);
</script>

<div class={cn('bg-primary/20 relative h-2 w-full overflow-hidden rounded-full', className)}>
	<div class="bg-foreground/30 absolute top-0 h-full w-px" style="left: {zeroPercentage}%"></div>

	<div
		title="Value: {value}"
		class={cn(
			'absolute top-0 h-full rounded-full transition-all',
			isNegative ? 'bg-destructive' : 'bg-primary'
		)}
		style="left: {left}%; width: {width}%"
	></div>
</div>
