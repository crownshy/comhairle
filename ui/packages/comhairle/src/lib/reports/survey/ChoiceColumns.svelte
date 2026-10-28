<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';
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

<ul class="my-auto flex items-stretch gap-3">
	{#each data as row, i (i)}
		{@const value = Number(row[y])}
		<li class="flex min-w-0 flex-1 flex-col items-center gap-2">
			<span class="text-muted-foreground whitespace-nowrap tabular-nums">
				<strong class="text-foreground">{value}</strong>{#if showShare}
					({share(value)}%){/if}
			</span>
			<div
				class="bg-muted flex h-40 w-full max-w-14 items-end overflow-hidden rounded-sm"
				aria-hidden="true"
			>
				<div class="bg-primary w-full" style:height="{(value / scale) * 100}%"></div>
			</div>
			<Tooltip.Root>
				<Tooltip.Trigger class="w-full">
					{#snippet child({ props })}
						<span
							{...props}
							tabindex="0"
							class="line-clamp-2 w-full text-center leading-snug [overflow-wrap:anywhere]"
						>
							{row[x]}
						</span>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content
					class="bg-background text-foreground max-w-xs border text-sm shadow-xl"
					arrowClasses="hidden">{row[x]}</Tooltip.Content
				>
			</Tooltip.Root>
		</li>
	{/each}
</ul>
