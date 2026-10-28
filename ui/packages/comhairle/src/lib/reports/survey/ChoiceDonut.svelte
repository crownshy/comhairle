<script lang="ts">
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';

	type Props = {
		data: Record<string, unknown>[];
		x: string;
		y: string;
	};

	let { data, x, y }: Props = $props();

	// Shades of the theme's primary blue, lightness spread out so neighbouring slices stay distinguishable.
	const palette = [
		'var(--primary)',
		'oklch(from var(--primary) calc(l + 0.25) calc(c * 0.7) h)',
		'oklch(from var(--primary) calc(l - 0.18) c h)',
		'oklch(from var(--primary) calc(l + 0.4) calc(c * 0.4) h)',
		'oklch(from var(--primary) calc(l + 0.12) c h)',
		'oklch(from var(--primary) calc(l - 0.3) calc(c * 0.8) h)'
	];

	const total = $derived(data.reduce((sum, row) => sum + Number(row[y]), 0));
	const share = (value: number) => (total === 0 ? 0 : Math.round((value / total) * 100));
</script>

<div class="flex flex-1 flex-wrap items-center gap-x-12 gap-y-6">
	<div class="w-56 shrink-0">
		<Doughnut {data} key={x} value={y} height={200} legend={false} cRange={palette} />
	</div>
	<ul class="flex min-w-60 flex-1 flex-col gap-2">
		{#each data as row, i (i)}
			{@const value = Number(row[y])}
			<li class="flex items-center gap-3">
				<span
					class="size-3.5 shrink-0 rounded-xs"
					style:background={palette[i % palette.length]}
					aria-hidden="true"
				></span>
				<span class="flex-1">{row[x]}</span>
				<span class="text-muted-foreground tabular-nums">
					<strong class="text-foreground">{value}</strong>
					({share(value)}%)
				</span>
			</li>
		{/each}
	</ul>
</div>
