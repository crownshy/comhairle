<script module lang="ts">
	import type { HeyFormFieldKind } from '$lib/tools/heyform/utils';

	export type View = 'vertical' | 'horizontal' | 'donut';

	export function defaultView(kind: HeyFormFieldKind, optionCount: number): View {
		if (kind === 'ranking' || kind === 'matrix') return 'horizontal';
		if (optionCount <= 3) return 'donut';
		return 'vertical';
	}
</script>

<script lang="ts">
	import Doughnut from '$lib/components/Charts/Doughnut.svelte';
	import BarChart from '$lib/components/Charts/BarChart.svelte';
	import Switcher from '$lib/components/Switcher.svelte';
	import { type Icon } from 'lucide-svelte';
	import { ArrowDownWideNarrow, ChartNoAxesColumn, ChartPie } from 'lucide-svelte';
	import { type ComponentProps, type ComponentType } from 'svelte';

	type Props = ComponentProps<typeof BarChart> & { kind: HeyFormFieldKind; view?: View };

	let {
		orientation: initialOrientation,
		data,
		x,
		y,
		kind,
		view = $bindable(),
		...props
	}: Props = $props();

	const currentView = $derived(
		view ?? initialOrientation ?? defaultView(kind, data?.length ?? 0)
	);
	const isScored = $derived(kind === 'ranking' || kind === 'matrix');
	// Slices stop being comparable past six; scored questions aren't shares of people.
	const donutAllowed = $derived(!isScored && (data?.length ?? 0) <= 6);

	let sortedData = $state<Props['data'] | null>(null);

	$effect(() => {
		if (currentView === 'horizontal' && sortedData === null) {
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
		initiallySelected={currentView}
		options={[
			{
				id: 'vertical',
				content: VIcon,
				aria: 'Columns'
			},
			{
				id: 'horizontal',
				content: HIcon,
				aria: 'Bars'
			},
			...(donutAllowed ? [{ id: 'donut', content: DIcon, aria: 'Donut' }] : [])
		]}
		onswitch={(id) => (view = id as View)}
	/>
</div>
{#if currentView === 'donut'}
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
		data={currentView === 'vertical' ? data : (sortedData ?? data)}
		{x}
		{y}
		orientation={currentView}
		{...props}
	/>
{/if}
