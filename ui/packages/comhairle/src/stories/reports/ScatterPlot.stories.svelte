<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ScatterPlot, { type ScatterPoint } from '$lib/reports/ScatterPlot.svelte';

	const { Story } = defineMeta({
		title: 'Components/reports/ScatterPlot',
		component: ScatterPlot,
		tags: ['autodocs'],
		argTypes: {
			xAxisLabel: { control: 'text' },
			yAxisLabel: { control: 'text' },
			height: { control: 'text' }
		}
	});

	const samplePoints: ScatterPoint[] = [
		{ id: '1', x: 2, y: 8 },
		{ id: '2', x: 4, y: 1 },
		{ id: '3', x: 8, y: 2 },
		{ id: '4', x: 3, y: 3 },
		{ id: '5', x: 7, y: 9 }
	];
</script>

<Story
	name="Default"
	args={{
		xAxisLabel: 'How important is this to you?',
		yAxisLabel: 'How satisfied are you with this?',
		xDomain: [0, 10],
		yDomain: [0, 10],
		points: samplePoints
	}}
>
	{#snippet children(args)}
		<div>
			<ScatterPlot {...args} />
		</div>
	{/snippet}
</Story>

<Story
	name="Empty"
	args={{
		xAxisLabel: 'How important is this to you?',
		yAxisLabel: 'How satisfied are you with this?',
		xDomain: [0, 10],
		yDomain: [0, 10],
		points: []
	}}
/>

<Story
	name="Negative domain"
	args={{
		xAxisLabel: 'Sentiment',
		yAxisLabel: 'Engagement',
		xDomain: [-10, 10],
		yDomain: [-10, 10],
		points: [
			{ id: '1', x: -5, y: 5 },
			{ id: '2', x: 0, y: 0 },
			{ id: '3', x: 5, y: -5 },
			{ id: '4', x: -8, y: -8 },
			{ id: '5', x: 8, y: 8 }
		]
	}}
/>

<Story
	name="Custom height"
	args={{
		xAxisLabel: 'How important is this to you?',
		yAxisLabel: 'How satisfied are you with this?',
		xDomain: [0, 10],
		yDomain: [0, 10],
		height: 200,
		points: samplePoints
	}}
/>
