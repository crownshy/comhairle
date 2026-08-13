<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import HorizontalBarChart from '$lib/components/Charts/HorizontalBarChart.svelte';
	import VerticalBarChart from '$lib/components/Charts/VerticalBarChart.svelte';

	interface Props extends ComponentProps<typeof HorizontalBarChart> {
		orientation?: 'horizontal' | 'vertical';
	}

	let { orientation = 'vertical', x, y, ...props }: Props = $props();
</script>

{#if orientation === 'vertical'}
	<VerticalBarChart {x} {y} {...props} />
{/if}
{#if orientation === 'horizontal'}
	<HorizontalBarChart x={y} y={x} {...props} />
{/if}
