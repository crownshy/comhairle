<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { LineChart } from 'layerchart';
	import { ArrowRight } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Bar = {
		label: string;
		/** Full name for screen readers and the hover title. */
		fullLabel: string;
		/** 0–100. */
		percent: number;
		/** Draws this bar in the attention colour. */
		flagged?: boolean;
	};

	type Props = {
		eyebrow: string;
		question: string;
		value: string;
		unit: string;
		/** Line sparkline. Ignored when `bars` is given. */
		trend?: number[];
		trendLabel: string;
		/** Small bar chart instead of the sparkline, e.g. finish rate per step. */
		bars?: Bar[];
		/** Any custom visual; replaces both the sparkline and the bars. */
		visual?: Snippet;
		actionLabel: string;
		actionHref: string;
		/** Opens the tab with the full detail for this question. */
		onShowDetails: () => void;
		/** Name of the tab that holds the detail, e.g. "Journey". */
		detailsTab: string;
	};

	let {
		eyebrow,
		question,
		value,
		unit,
		trend,
		trendLabel,
		bars,
		visual,
		actionLabel,
		actionHref,
		onShowDetails,
		detailsTab
	}: Props = $props();

	let trendData = $derived((trend ?? []).map((point, index) => ({ index, point })));
</script>

<Card.Root class="flex flex-col gap-4">
	<Card.Header class="gap-3">
		<div class="flex items-center justify-between gap-2">
			<span class="text-primary text-sm font-semibold tracking-wide uppercase">{eyebrow}</span
			>
		</div>
		<Card.Title class="text-muted-foreground text-sm">{question}</Card.Title>
	</Card.Header>
	<Card.Content class="flex flex-col gap-4">
		<div class="flex items-baseline gap-2">
			<span class="shrink-0 text-4xl leading-none font-bold whitespace-nowrap">{value}</span>
			<span class="text-muted-foreground text-base">{unit}</span>
		</div>
		{#if visual}
			{@render visual()}
		{:else if bars}
			<div class="flex flex-col gap-1.5">
				<span class="text-muted-foreground text-sm">{trendLabel}</span>
				<ul
					class="grid h-28 gap-1.5"
					style:grid-template-columns="repeat({bars.length}, minmax(0, 1fr))"
				>
					{#each bars as bar (bar.fullLabel)}
						<li
							class="flex min-w-0 flex-col items-center gap-1"
							title="{bar.fullLabel}: {bar.percent}%"
						>
							<span
								class={cn(
									'text-sm font-semibold tabular-nums',
									bar.flagged && 'text-amber-900'
								)}
							>
								{bar.percent}%
							</span>
							<div
								class="bg-muted flex w-full flex-1 items-end overflow-hidden rounded-md"
								aria-hidden="true"
							>
								<div
									class={cn(
										'w-full rounded-md',
										bar.flagged ? 'bg-amber-500' : 'bg-primary'
									)}
									style:height="{bar.percent}%"
								></div>
							</div>
							<span class="text-muted-foreground w-full truncate text-center text-sm">
								<span aria-hidden="true">{bar.label}</span>
								<span class="sr-only">{bar.fullLabel}</span>
							</span>
						</li>
					{/each}
				</ul>
			</div>
		{:else}
			<div class="h-14" role="img" aria-label={trendLabel}>
				<LineChart
					data={trendData}
					x="index"
					y="point"
					axis={false}
					grid={false}
					highlight={false}
					tooltipContext={false}
					series={[{ key: 'point', color: 'var(--primary)' }]}
					props={{ spline: { strokeWidth: 2.5 } }}
				/>
			</div>
		{/if}
	</Card.Content>
	<Card.Footer class="mt-auto flex flex-wrap items-center justify-between gap-3 border-t pt-4">
		<Button href={actionHref} variant="secondary" class="rounded-full">{actionLabel}</Button>
		<button
			type="button"
			onclick={onShowDetails}
			class="text-primary inline-flex items-center gap-1 text-base font-semibold hover:underline"
		>
			{detailsTab}
			<ArrowRight class="size-4" aria-hidden="true" />
		</button>
	</Card.Footer>
</Card.Root>
