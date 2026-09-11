<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import HorizontalBarChart from '$lib/components/Charts/HorizontalBarChart.svelte';
	import VerticalBarChart from '$lib/components/Charts/VerticalBarChart.svelte';

	interface Props extends ComponentProps<typeof HorizontalBarChart> {
		orientation?: 'horizontal' | 'vertical';
	}

	let { orientation = 'vertical', config, ...props }: Props = $props();

	function swapAxes(config: Props['config']): Props['config'] {
		switch (config.type) {
			case 'normal':
				return {
					...config,
					x: config.y,
					y: config.x
				};
			case 'xSeries':
				return {
					...config,
					type: 'ySeries',
					x: undefined,
					y: config.x
				};
			case 'ySeries':
				return {
					...config,
					type: 'xSeries',
					x: config.y,
					y: undefined
				};
		}
	}
</script>

{#if orientation === 'vertical'}
	<VerticalBarChart {config} {...props} />
{/if}
{#if orientation === 'horizontal'}
	<HorizontalBarChart config={swapAxes(config)} {...props} />
{/if}
