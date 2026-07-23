<!--
	@component A single horizontal stacked vote bar: agreed / disagreed / passed /
	not-voted segments across the full width, with an optional row label.

	Dumb leaf: it takes four already-computed percentages (0-100, summing to ~100
	over the group's members, so the not-voted remainder shows) and renders them.
	All colour comes from the shared `--vote-*` theme tokens, so it themes with the
	rest of the report. The caller decides what the four numbers mean (overall vs a
	single opinion group) and how they were computed.
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

	// Segments in consensus-first order. `border` flags the not-voted segment so it
	// stays visible on a card background (it is near-white by token).
	const segments = $derived([
		{ key: 'agreed', pct: agreed, color: 'var(--vote-agreed)', border: false },
		{ key: 'disagreed', pct: disagreed, color: 'var(--vote-disagreed)', border: false },
		{ key: 'passed', pct: passed, color: 'var(--vote-passed)', border: false },
		{ key: 'notVoted', pct: notVoted, color: 'var(--vote-not-voted)', border: true }
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
					class:border-border={s.border}
					class:border-l={s.border}
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
