<!--
	@component A single horizontal vote bar: agreed / disagreed / passed segments
	packed left-to-right inside an outlined track, with an optional row label.

	Not-voted is NOT a drawn segment: it is the empty tail of the track (the three
	segments only fill their combined share, and the border outlines the whole bar
	so the remainder still reads as part of it). This matches the design, where the
	not-voted remainder is empty outlined space with no label.

	Dumb leaf: it takes four already-computed percentages (0-100, summing to ~100
	over the group's members). All fill colour comes from the shared `--vote-*`
	theme tokens. The caller decides what the four numbers mean (overall vs a single
	opinion group) and how they were computed.
-->
<script lang="ts">
	type Props = {
		/** Row label, e.g. "OVERALL" or "GROUP A". Omit for a bare bar. */
		label?: string;
		agreed: number;
		disagreed: number;
		passed: number;
		notVoted: number;
	};

	let { label, agreed, disagreed, passed, notVoted }: Props = $props();

	// Consensus-first order. Not-voted is intentionally absent: it is the empty tail
	// of the outlined track, not a drawn segment.
	const segments = $derived([
		{ key: 'agreed', pct: agreed, color: 'var(--vote-agreed)' },
		{ key: 'disagreed', pct: disagreed, color: 'var(--vote-disagreed)' },
		{ key: 'passed', pct: passed, color: 'var(--vote-passed)' }
	]);

	// Only label a segment when it is wide enough to fit the text.
	const LABEL_MIN_PCT = 8;
</script>

<div class="flex items-center gap-2">
	{#if label}
		<span class="text-foreground w-16 shrink-0 text-xs font-medium">{label}</span>
	{/if}
	<div
		class="border-border flex h-3.5 min-w-0 flex-1 overflow-hidden rounded-lg border"
		role="img"
		aria-label={`${Math.round(agreed)}% agreed, ${Math.round(disagreed)}% disagreed, ${Math.round(
			passed
		)}% passed, ${Math.round(notVoted)}% not voted`}
	>
		{#each segments as s (s.key)}
			{#if s.pct > 0}
				<div
					class="flex items-center justify-end overflow-hidden"
					style="width: {s.pct}%; background: {s.color};"
				>
					{#if s.pct >= LABEL_MIN_PCT}
						<span class="text-foreground px-1 text-xs font-medium tabular-nums"
							>{Math.round(s.pct)}%</span
						>
					{/if}
				</div>
			{/if}
		{/each}
	</div>
</div>
