<!--
	@component A single horizontal vote bar: agreed / disagreed / passed /
	not-voted drawn as rounded pills packed left-to-right across the full width,
	with an optional row label.

	Every segment is filled and the bar is full-width: it stretches to fill
	whatever the caller gives it. Passed is a solid grey; not-voted is the empty
	(near-white) tail carrying a thin outline so it stays distinct from passed even
	when the two greys are close (e.g. scot-gov).

	The tooltip follows the cursor. bits-ui's Tooltip normally anchors to the
	trigger element and floating-ui does NOT recompute on plain mouse movement, so
	on its own the bubble sits centred over the trigger and cannot track the
	pointer. The way around it (without leaving bits-ui) is `customAnchor`: it
	accepts a virtual "measurable" (anything with getBoundingClientRect). We hand
	Tooltip.Content a zero-size virtual anchor pinned to the cursor and reassign it
	on every pointermove; bits-ui reacts to the new reference and re-solves the
	position, so the bubble rides along and shows whichever slice is under the
	cursor.

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

	const summary = $derived(
		`${Math.round(agreed)}% agreed, ${Math.round(disagreed)}% disagreed, ` +
			`${Math.round(passed)}% passed, ${Math.round(notVoted)}% not voted`
	);

	// The slice under the cursor, tracked by key so it survives prop-driven
	// re-derivations of `visible`; falls back to the first slice (e.g. on keyboard
	// focus, when there is no pointer position to read).
	let activeKey = $state<string | null>(null);
	const active = $derived(visible.find((s) => s.key === activeKey) ?? visible[0]);

	// Virtual anchor pinned to the cursor. Reassigned on each move so bits-ui
	// re-solves the tooltip position (see the component doc above).
	let cursorAnchor = $state<{ getBoundingClientRect: () => DOMRect } | null>(null);

	function trackCursor(event: PointerEvent) {
		const track = event.currentTarget as HTMLElement;
		const rect = track.getBoundingClientRect();
		const ratio = ((event.clientX - rect.left) / rect.width) * 100;

		let acc = 0;
		let hit = visible[visible.length - 1];
		for (const s of visible) {
			acc += s.pct;
			if (ratio <= acc) {
				hit = s;
				break;
			}
		}
		activeKey = hit.key;

		// Follow the cursor's x; keep the y at the bar's middle so the bubble sits a
		// steady distance above the bar instead of jittering with tiny y wobble.
		const x = event.clientX;
		const y = rect.top + rect.height / 2;
		cursorAnchor = {
			getBoundingClientRect: () => DOMRect.fromRect({ x, y, width: 0, height: 0 })
		};
	}
</script>

<Tooltip.Provider delayDuration={100}>
	<div class="flex items-center gap-3">
		{#if label}
			<span class="text-muted-foreground w-14 shrink-0 text-xs font-medium">{label}</span>
		{/if}
		<Tooltip.Root>
			<Tooltip.Trigger
				aria-label={summary}
				onpointermove={trackCursor}
				class="flex h-2.5 min-w-0 flex-1 cursor-default items-stretch border-0 bg-transparent p-0"
			>
				{#each visible as s, i (s.key)}
					<span
						class="h-full min-w-0 rounded-full"
						style="flex: {s.pct} {s.pct} 0; background: {s.color}; z-index: {visible.length -
							i}; margin-left: {i > 0 ? `-${OVERLAP}` : '0'};{s.outline
							? ' box-shadow: inset 0 0 0 1px var(--vote-not-voted-border);'
							: ''}"
					></span>
				{/each}
			</Tooltip.Trigger>
			<Tooltip.Content customAnchor={cursorAnchor} sideOffset={8}>
				{active?.label} · {Math.round(active?.pct ?? 0)}%
			</Tooltip.Content>
		</Tooltip.Root>
	</div>
</Tooltip.Provider>
