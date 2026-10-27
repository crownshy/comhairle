<script lang="ts">
	type Props = {
		data: Record<string, unknown>[];
		x: string;
		y: string;
		// Scores (rankings, matrices) aren't shares of people, so no percentage.
		showShare: boolean;
	};

	let { data, x, y, showShare }: Props = $props();

	const total = $derived(data.reduce((sum, row) => sum + Number(row[y]), 0));
	const max = $derived(Math.max(1, ...data.map((row) => Number(row[y]))));
	const scale = $derived(showShare ? Math.max(total, 1) : max);
	const share = (value: number) => (total === 0 ? 0 : Math.round((value / total) * 100));
</script>

<ul class="flex flex-col gap-3">
	{#each data as row, i (i)}
		{@const value = Number(row[y])}
		<li class="grid grid-cols-[minmax(7rem,11rem)_1fr_5rem] items-center gap-4">
			<span>{row[x]}</span>
			<div class="bg-muted h-6 overflow-hidden rounded-sm" aria-hidden="true">
				<div class="bg-primary h-full" style:width="{(value / scale) * 100}%"></div>
			</div>
			<span class="text-muted-foreground text-right tabular-nums">
				<strong class="text-foreground">{value}</strong>{#if showShare}
					({share(value)}%){/if}
			</span>
		</li>
	{/each}
</ul>
