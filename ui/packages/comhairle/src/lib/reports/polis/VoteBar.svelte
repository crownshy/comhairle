<!--
	@component A single horizontal vote bar: agreed / disagreed / passed /
	not-voted drawn as rounded pills packed left-to-right across the full width,
	with an optional row label.

	Every segment is filled and the bar is full-width: it stretches to fill
	whatever the caller gives it. Passed is a solid grey; not-voted is the empty
	(near-white) tail carrying a thin outline so it stays distinct from passed even
	when the two greys are close (e.g. scot-gov). Hovering (or focusing) a segment
	shows a tooltip with its category and percentage.

	Dumb leaf: it takes four already-computed percentages (0-100, summing to ~100
	over the group's members). All fill colour comes from the shared `--vote-*`
	theme tokens. The caller decides what the four numbers mean (overall vs a single
	opinion group) and how they were computed.
-->
<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';

	type Props = {
		/** Row label, e.g. "OVERALL" or "GROUP A". Omit for a bare bar. */
		label?: string;
		agreed: number;
		disagreed: number;
		passed: number;
		notVoted: number;
	};

	let { label, agreed, disagreed, passed, notVoted }: Props = $props();

	// Consensus-first order. Passed is a solid grey; not-voted is the near-white
	// tail. `outline` gives not-voted a thin inset border so it never blurs into
	// passed (their greys can sit close, e.g. scot-gov) or into a white card.
	const segments = $derived([
		{
			key: 'agreed',
			label: 'Agreed',
			pct: agreed,
			color: 'var(--vote-agreed)',
			outline: false
		},
		{
			key: 'disagreed',
			label: 'Disagreed',
			pct: disagreed,
			color: 'var(--vote-disagreed)',
			outline: false
		},
		{
			key: 'passed',
			label: 'Passed',
			pct: passed,
			color: 'var(--vote-passed)',
			outline: false
		},
		{
			key: 'notVoted',
			label: 'Not voted',
			pct: notVoted,
			color: 'var(--vote-not-voted)',
			outline: true
		}
	]);

	// Only the non-empty slices are drawn. Each is a rounded pill sized by
	// flex-grow in proportion to its percentage, so every bar fills the whole
	// track and all rows end at the same right edge regardless of how many slices
	// they have. Consecutive pills overlap by their radius (`OVERLAP`) so the caps
	// nest like the design; earlier (leftward) slices stack on top via a
	// descending z-index. The overlap steals a little width, but flex-grow hands it
	// straight back, keeping totals equal.
	const visible = $derived(segments.filter((s) => s.pct > 0));
	const OVERLAP = '0.3125rem'; // half the h-2.5 track height, i.e. one cap radius
</script>

<Tooltip.Provider delayDuration={100}>
	<div class="flex items-center gap-3">
		{#if label}
			<span class="text-muted-foreground w-14 shrink-0 text-xs font-medium">{label}</span>
		{/if}
		<div
			class="flex h-2.5 min-w-0 flex-1 items-stretch"
			role="img"
			aria-label={`${Math.round(agreed)}% agreed, ${Math.round(disagreed)}% disagreed, ${Math.round(
				passed
			)}% passed, ${Math.round(notVoted)}% not voted`}
		>
			{#each visible as s, i (s.key)}
				<Tooltip.Root>
					<Tooltip.Trigger
						aria-label={`${s.label} ${Math.round(s.pct)}%`}
						class="m-0 h-full min-w-0 cursor-default rounded-full border-0 p-0"
						style="flex: {s.pct} {s.pct} 0; background: {s.color}; z-index: {visible.length -
							i}; margin-left: {i > 0 ? `-${OVERLAP}` : '0'};{s.outline
							? ' box-shadow: inset 0 0 0 1px var(--vote-not-voted-border);'
							: ''}"
					/>
					<Tooltip.Content>{s.label} · {Math.round(s.pct)}%</Tooltip.Content>
				</Tooltip.Root>
			{/each}
		</div>
	</div>
</Tooltip.Provider>
