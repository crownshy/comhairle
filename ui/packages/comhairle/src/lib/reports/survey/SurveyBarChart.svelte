<script lang="ts">
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Switcher from '$lib/components/Switcher.svelte';
	import type { HeyFormFieldKind } from '$lib/tools/heyform/utils';
	import { type Icon } from 'lucide-svelte';
	import { ArrowDownWideNarrow, ChartNoAxesColumn } from 'lucide-svelte';
	import { type ComponentProps, type ComponentType } from 'svelte';

	type Props = ComponentProps<typeof BarChart> & { kind: HeyFormFieldKind };

	let { orientation: initialOrientation, config, kind, ...props }: Props = $props();

	let orientation = $derived.by<Props['orientation']>(() => {
		if (initialOrientation) return initialOrientation;
		if (kind === 'ranking' || kind === 'matrix') return 'horizontal';
		return 'vertical';
	});

	let sortedData = $state<Props['config']['data'] | null>(null);

	$effect(() => {
		if (orientation === 'horizontal' && sortedData === null) {
			sortedData = config.data?.toSorted((a, b) => {
				if (!config.y) {
					return 0;
				}
				return Number(b[config.y]) - Number(a[config.y]);
			});
		}
	});
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
		onswitch={(id) => (orientation = id as Props['orientation'])}
	/>
</div>
<BarChart
	{...props}
	config={{
		...config,
		data: orientation === 'vertical' ? config.data : (sortedData ?? config.data)
	}}
	{orientation}
/>
