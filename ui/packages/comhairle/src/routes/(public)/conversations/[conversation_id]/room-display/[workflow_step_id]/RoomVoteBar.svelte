<!--
	@component PROTOTYPE. One vote bar at room scale: the label and the agree share as a
	big number on one line, a fat segmented bar underneath. Built to sit in a row of
	columns (Overall, Group A, Group B) so one statement takes two lines of height, not
	four.

	The report's `VoteBar` is a desk component (10px tall, 12px label, a tooltip that
	follows the cursor). None of that survives a projector, and a tooltip is exactly
	what an ambient display must not rely on, so the number the room most wants is
	printed instead of hidden behind a hover.

	Dumb leaf: four percentages over members present, same tokens as the report bars.
-->
<script lang="ts">
	type Props = {
		label: string;
		agreed: number;
		disagreed: number;
		passed: number;
		notVoted: number;
	};

	let { label, agreed, disagreed, passed, notVoted }: Props = $props();

	const segments = $derived(
		[
			{ key: 'agreed', pct: agreed, color: 'var(--vote-agreed)', outlined: false },
			{ key: 'disagreed', pct: disagreed, color: 'var(--vote-disagreed)', outlined: false },
			{ key: 'passed', pct: passed, color: 'var(--vote-passed)', outlined: false },
			{ key: 'notVoted', pct: notVoted, color: 'var(--vote-not-voted)', outlined: true }
		].filter((s) => s.pct > 0)
	);

	const summary = $derived(
		`${label}: ${Math.round(agreed)}% agreed, ${Math.round(disagreed)}% disagreed, ` +
			`${Math.round(passed)}% passed, ${Math.round(notVoted)}% not voted`
	);
</script>

<div class="flex min-w-0 flex-col gap-1.5" role="img" aria-label={summary}>
	<div class="flex items-baseline justify-between gap-3">
		<span class="text-muted-foreground truncate text-lg font-medium lg:text-xl">{label}</span>
		<span
			class="text-foreground shrink-0 text-xl font-bold whitespace-nowrap tabular-nums lg:text-2xl"
		>
			{Math.round(agreed)}%
			<span class="text-muted-foreground text-base font-medium">agree</span>
		</span>
	</div>
	<div class="bg-muted flex h-4 overflow-hidden rounded-full lg:h-5">
		{#each segments as s (s.key)}
			<span
				class="h-full transition-[width] duration-500"
				style="width: {s.pct}%; background: {s.color};{s.outlined
					? ' box-shadow: inset 0 0 0 1px var(--vote-not-voted-border);'
					: ''}"
			></span>
		{/each}
	</div>
</div>
