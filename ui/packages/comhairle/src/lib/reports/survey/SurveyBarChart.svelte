<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Switcher from '$lib/components/Switcher.svelte';
	import type { HeyFormFieldKind } from '$lib/tools/heyform/utils';
	import { type Icon } from 'lucide-svelte';
	import { ArrowDownWideNarrow, ChartNoAxesColumn } from 'lucide-svelte';
	import type { ComponentProps, ComponentType } from 'svelte';

	type Props = ComponentProps<typeof BarChart> & { kind: HeyFormFieldKind };

	let { orientation: initialOrientation, data, x, y, kind, ...props }: Props = $props();

	let orientation = $derived.by<Props['orientation']>(() => {
		if (initialOrientation) return initialOrientation;
		if (kind === 'ranking') return 'horizontal';
		return 'vertical';
	});

	let sortedData = $state<Props['data'] | null>(null);
</script>

{#snippet icon(Icon: ComponentType<Icon>)}
	<Icon class="size-6" />
{/snippet}

{#snippet VIcon()}
	{@render icon(ChartNoAxesColumn)}
{/snippet}

{#snippet HIcon()}
	{@render icon(ArrowDownWideNarrow)}
{/snippet}

<div class="flex flex-row justify-end">
	<Switcher
		initiallySelected={orientation}
		options={[
			{
				id: 'vertical',
				content: VIcon,
				aria: 'Sort by label'
			},
			{
				id: 'horizontal',
				content: HIcon,
				aria: 'Sort by value'
			}
		]}
		onswitch={(id) => {
			orientation = id as Props['orientation'];

			if (orientation === 'horizontal' && sortedData === null) {
				sortedData = data?.toSorted((a, b) => Number(b[y]) - Number(a[y]));
			}
		}}
	/>
</div>
<BarChart
	data={orientation === 'vertical' ? data : (sortedData ?? data)}
	{x}
	{y}
	{orientation}
	{...props}
/>
