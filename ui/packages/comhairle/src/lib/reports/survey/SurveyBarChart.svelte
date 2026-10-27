<script lang="ts">
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Switcher from '$lib/components/Switcher.svelte';
	import type { HeyFormFieldKind } from '$lib/tools/heyform/utils';
	import { type Icon } from 'lucide-svelte';
	import { ArrowDownWideNarrow, ChartNoAxesColumn, ChartPie } from 'lucide-svelte';
	import { type ComponentProps, type ComponentType } from 'svelte';

	type Props = ComponentProps<typeof BarChart> & { kind: HeyFormFieldKind };

	let { orientation: initialOrientation, data, x, y, kind, ...props }: Props = $props();

	type View = 'vertical' | 'horizontal' | 'donut';

	const isScored = $derived(kind === 'ranking' || kind === 'matrix');
	// Slices stop being comparable past six; scored questions aren't shares of people.
	const donutAllowed = $derived(!isScored && (data?.length ?? 0) <= 6);

	let view = $derived.by<View>(() => {
		if (initialOrientation) return initialOrientation;
		if (isScored) return 'horizontal';
		if ((data?.length ?? 0) <= 3) return 'donut';
		return 'vertical';
	});

	let sortedData = $state<Props['data'] | null>(null);

	$effect(() => {
		if (view === 'horizontal' && sortedData === null) {
			sortedData = data?.toSorted((a, b) => Number(b[y]) - Number(a[y]));
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

{#snippet DIcon()}
	{@render icon(ChartPie)}
{/snippet}

<div class="flex flex-row justify-end">
	<Switcher
		initiallySelected={view}
		options={[
			{
				id: 'vertical',
				content: VIcon,
				aria: 'Columns'
			},
			{
				id: 'horizontal',
				content: HIcon,
				aria: 'Row'
			},
			...(donutAllowed ? [{ id: 'donut', content: DIcon, aria: 'Donut' }] : [])
		]}
		onswitch={(id) => (view = id as View)}
	/>
</div>
{#if view === 'donut'}
	<div class="flex flex-1 flex-col justify-center">
		<Doughnut
			{data}
			key={x}
			value={y}
			height={200}
			legend={{ placement: 'right', orientation: 'vertical' }}
		/>
	</div>
{:else}
	<BarChart
		data={view === 'vertical' ? data : (sortedData ?? data)}
		{x}
		{y}
		orientation={view}
		{...props}
	/>
{/if}
