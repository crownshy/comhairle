<!--
	@component One statement's vote bar at room scale, with the agree percentage printed
	instead of hidden in a tooltip. Values are percentages.
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
