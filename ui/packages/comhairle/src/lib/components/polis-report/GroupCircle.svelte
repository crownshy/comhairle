<script lang="ts">
	import * as HoverCard from '$lib/components/ui/hover-card';

	/**
	 * Compact agree% visualization for one group on a single statement. A colored
	 * arc whose length is the group's agree% and whose color grades the extent of
	 * agreement — low (< 33%) destructive, mid (33–66%) muted, high (66%+) primary.
	 * The non-agree remainder shows as the neutral track. Hovering reveals the full
	 * vote breakdown (agree/disagree/pass) for that group + statement.
	 */
	interface Props {
		/** Group letter (A, B, …). Shown in the tooltip header and optional label. */
		label?: string;
		/**
		 * Agree% driving the arc length + center number, 0–100. This is the value
		 * already reflecting the section's include/exclude-passes toggle, so the arc
		 * matches the "Agree %" column.
		 */
		agreePct: number;
		/** Raw vote counts — power the tooltip's honest full breakdown (always incl. passes) and N. */
		agrees: number;
		disagrees: number;
		passes: number;
		size?: 'sm' | 'md';
		/** Show the group label beneath the ring. Off in dense statement rows. */
		showLabel?: boolean;
	}

	let {
		label = '',
		agreePct,
		agrees,
		disagrees,
		passes,
		size = 'sm',
		showLabel = true
	}: Props = $props();

	const clamped = $derived(Math.min(100, Math.max(0, agreePct)));

	// Extent-of-agreement color buckets mapped to theme tokens: < 33% destructive,
	// 33–66% muted-foreground, 66%+ primary. `currentColor` on the arc stroke lets
	// the text-* class drive it.
	const arcColor = $derived(
		clamped < 33 ? 'text-destructive' : clamped <= 66 ? 'text-muted-foreground' : 'text-primary'
	);

	// SVG geometry. viewBox is 40×40; the ring sits inside the stroke so r leaves
	// half the stroke width on each side. Rotated -90° so the arc starts at 12
	// o'clock and grows clockwise.
	const R = 17.5;
	const CIRC = 2 * Math.PI * R;
	const dash = $derived((clamped / 100) * CIRC);

	const ringSize = $derived(size === 'md' ? 'size-11' : 'size-10');

	// Tooltip: the TRUE full breakdown over all votes cast (incl. passes), so the
	// three rows always sum to ~100% regardless of the exclude-passes toggle.
	const total = $derived(agrees + disagrees + passes);
	const pct = (n: number) => (total > 0 ? Math.round((n / total) * 100) : 0);
</script>

<!-- Vote-breakdown on hover. HoverCard (bits-ui) portals the panel to the body and
     uses Floating UI collision handling, so it flips/shifts to stay on-screen and
     never extends or is clipped by the page — unlike the old CSS-only tooltip that
     ran off the right edge on the last column. -->
<HoverCard.Root openDelay={80} closeDelay={80}>
	<HoverCard.Trigger
		class="flex cursor-default flex-col items-center gap-0.5 outline-none"
		aria-label={`Group ${label} vote breakdown`}
	>
		<div class={`relative ${ringSize}`}>
			<svg class="size-full -rotate-90" viewBox="0 0 40 40" aria-hidden="true">
				<circle
					class="text-border"
					cx="20"
					cy="20"
					r={R}
					fill="none"
					stroke="currentColor"
					stroke-width="5"
				/>
				<circle
					class={arcColor}
					cx="20"
					cy="20"
					r={R}
					fill="none"
					stroke="currentColor"
					stroke-width="5"
					stroke-linecap="round"
					stroke-dasharray={`${dash} ${CIRC - dash}`}
				/>
			</svg>
			<span
				class="text-foreground absolute inset-0 flex items-center justify-center text-xs leading-none font-bold"
			>
				{Math.round(clamped)}
			</span>
		</div>

		{#if showLabel && label}
			<span class="text-muted-foreground text-xs font-semibold uppercase">{label}</span>
		{/if}
	</HoverCard.Trigger>

	<HoverCard.Content side="top" align="end" sideOffset={8} class="w-60 p-5">
		<div class="text-popover-foreground text-base font-bold uppercase">
			Group {label} <span class="text-muted-foreground">(N={total})</span>
		</div>
		<dl class="text-muted-foreground mt-3 space-y-2">
			<div class="flex items-center justify-between text-base font-semibold">
				<dt>Agree</dt>
				<dd class="tabular-nums">{pct(agrees)}%</dd>
			</div>
			<div class="flex items-center justify-between text-base font-semibold">
				<dt>Disagree</dt>
				<dd class="tabular-nums">{pct(disagrees)}%</dd>
			</div>
			<div class="flex items-center justify-between text-base font-semibold">
				<dt>Pass</dt>
				<dd class="tabular-nums">{pct(passes)}%</dd>
			</div>
		</dl>
	</HoverCard.Content>
</HoverCard.Root>
