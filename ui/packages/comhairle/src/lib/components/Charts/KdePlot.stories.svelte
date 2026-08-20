<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import KdePlot from '$lib/components/Charts/KdePlot.svelte';

	const { Story } = defineMeta({
		title: 'Components/reports/KdePlot',
		component: KdePlot,
		tags: ['autodocs'],
		argTypes: {
			minLabel: { control: 'text' },
			maxLabel: { control: 'text' },
			category: { control: 'text' },
			maxX: { control: 'number' }
		}
	});

	// Roughly bell-shaped, centered around 5
	const centered = [
		4.8, 5.2, 4.5, 5.5, 5.0, 4.9, 5.1, 4.6, 5.4, 5.0, 4.7, 5.3, 5.0, 4.4, 5.6, 4.9, 5.1, 5.0,
		4.8, 5.2, 5.0, 4.6, 5.4, 5.0
	];

	// Clustered toward the low end
	const lowSkew = [
		1.2, 1.8, 2.0, 1.5, 2.3, 1.9, 1.1, 2.5, 1.7, 2.1, 1.4, 1.6, 2.2, 1.3, 2.0, 1.8, 1.5, 2.4,
		1.9, 1.7
	];

	// Clustered toward the high end
	const highSkew = [
		7.8, 8.2, 8.5, 7.5, 8.9, 8.1, 7.9, 8.6, 8.0, 7.6, 8.3, 8.8, 7.7, 8.4, 8.0, 8.7, 7.9, 8.2,
		8.5, 8.1
	];

	// Two distinct clusters
	const bimodal = [
		1.5, 2.0, 1.8, 2.2, 1.6, 1.9, 2.1, 1.7, 2.3, 1.4, 8.0, 7.6, 8.3, 7.9, 8.5, 7.7, 8.1, 8.4,
		7.8, 8.2
	];

	// Spread fairly evenly across the domain
	const wideSpread = [
		0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 1.0, 3.0, 5.0, 7.0, 9.0, 2.0, 4.0, 6.0,
		8.0, 0.8
	];
</script>

<Story
	name="Default"
	args={{
		category: 'satisfaction',
		data: { satisfaction: centered },
		minLabel: 'Not at all',
		maxLabel: 'Extremely',
		maxX: 10
	}}
/>

<Story
	name="No labels"
	args={{
		category: 'satisfaction',
		data: { satisfaction: centered },
		maxX: 10
	}}
/>

<Story
	name="Skewed low"
	args={{
		category: 'difficulty',
		data: { difficulty: lowSkew },
		minLabel: 'Easy',
		maxLabel: 'Hard',
		maxX: 10
	}}
/>

<Story
	name="Skewed high"
	args={{
		category: 'agreement',
		data: { agreement: highSkew },
		minLabel: 'Disagree',
		maxLabel: 'Agree',
		maxX: 10
	}}
/>

<Story
	name="Bimodal"
	args={{
		category: 'sentiment',
		data: { sentiment: bimodal },
		minLabel: 'Negative',
		maxLabel: 'Positive',
		maxX: 10
	}}
/>

<Story
	name="Wide spread"
	args={{
		category: 'confidence',
		data: { confidence: wideSpread },
		minLabel: 'Not confident',
		maxLabel: 'Very confident',
		maxX: 10
	}}
/>

<Story
	name="Custom domain"
	args={{
		category: 'score',
		data: { score: centered.map((v) => v * 5) },
		minLabel: '0',
		maxLabel: '50',
		maxX: 50
	}}
/>

<Story
	name="Without density line"
	args={{
		category: 'score',
		data: { score: centered.map((v) => v * 5) },
		minLabel: '0',
		maxLabel: '50',
		maxX: 50,
		options: { densityLine: false }
	}}
/>

<Story
	name="With wave outlineline"
	args={{
		category: 'score',
		data: { score: centered.map((v) => v * 5) },
		minLabel: '0',
		maxLabel: '50',
		maxX: 50,
		options: { outline: true, densityLine: false }
	}}
/>
