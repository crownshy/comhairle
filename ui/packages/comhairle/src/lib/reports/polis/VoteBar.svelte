<!--
	@component A single horizontal vote bar: agreed / disagreed / passed /
	not-voted drawn as rounded pills packed left-to-right across the full width, with
	an optional row label. Consecutive pills overlap by one cap radius so the rounded
	ends nest into each other like the design, instead of leaving a lens-shaped gap.

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

	const visible = $derived(segments.filter((s) => s.pct > 0));

	// How far each pill's rounded cap laps over the next one. Bump this for a more
	// pronounced nested overlap; drop to 0 for a flush segmented bar.
	const OVERLAP = '0.75rem';

	// Lay the pills out by absolute position rather than flex + negative margins:
	// each pill sits at its true cumulative percentage (`left`/`width`), so widths
	// stay exact and the last pill always lands on the right edge. To nest the
	// rounded caps, every pill except the last is widened by `OVERLAP` so it laps
	// over its right-hand neighbour; a descending z-index keeps the left pill on
	// top, so its rounded cap sits over the one after it (matching the design).
	const pills = $derived.by(() => {
		const total = visible.reduce((sum, s) => sum + s.pct, 0) || 1;
		let acc = 0;
		return visible.map((s, i) => {
			const left = (acc / total) * 100;
			acc += s.pct;
			const width = (acc / total) * 100 - left;
			return { ...s, left, width, isLast: i === visible.length - 1 };
		});
	});

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
				class="relative block h-2.5 min-w-0 flex-1 cursor-default border-0 bg-transparent p-0"
			>
				{#each pills as p, i (p.key)}
					<span
						class="absolute inset-y-0 rounded-full"
						style="left: {p.left}%; width: {p.isLast
							? `${p.width}%`
							: `calc(${p.width}% + ${OVERLAP})`}; z-index: {pills.length -
							i}; background: {p.color};{p.outline
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
