<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { ArrowDownWideNarrow, ChartNoAxesColumn } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof BarChart>;

	let { orientation: initialOrientation = 'vertical', data, x, y, ...props }: Props = $props();

	let orientation = $derived<Props['orientation']>(initialOrientation);

	let sortedData = $state<Props['data'] | null>(null);
</script>

<div class="flex flex-row justify-end">
	{#if orientation === 'vertical'}
		<Button
			variant="outline"
			class="rounded-md"
			aria-label="Sort by value"
			title="Sort by value"
			onclick={() => {
				orientation = 'horizontal';

				if (sortedData === null) {
					sortedData = data.toSorted((a, b) => Number(b[y]) - Number(a[y]));
				}
			}}
		>
			<ArrowDownWideNarrow class="size-6" />
		</Button>
	{:else}
		<Button
			variant="outline"
			class="rounded-md"
			aria-label="Sort by label"
			title="Sort by label"
			onclick={() => (orientation = 'vertical')}
		>
			<ChartNoAxesColumn class="size-6" />
		</Button>
	{/if}
</div>
<BarChart
	data={orientation === 'vertical' ? data : (sortedData ?? data)}
	{x}
	{y}
	{orientation}
	{...props}
/>
