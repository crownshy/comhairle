<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import DivergingProgress from '$lib/reports/DivergingProgress.svelte';

	const { Story } = defineMeta({
		title: 'Components/DivergingProgress',
		component: DivergingProgress,
		tags: ['autodocs'],
		argTypes: {
			value: { control: 'number' },
			min: { control: 'number' },
			max: { control: 'number' }
		}
	});
</script>

<Story name="Positive value" args={{ value: 6 }} />

<Story name="Negative value" args={{ value: -6 }} />

<Story name="Zero" args={{ value: 0 }} />

<Story name="Max value" args={{ value: 10 }} />

<Story name="Min value" args={{ value: -10 }} />

<Story name="Small positive" args={{ value: 0.5 }} />

<Story name="Small negative" args={{ value: -0.5 }} />

<Story
	name="Asymmetric domain"
	args={{
		value: 20,
		min: -50,
		max: 100
	}}
/>

<Story
	name="All positive domain"
	args={{
		value: 30,
		min: 0,
		max: 100
	}}
/>

<Story
	name="All negative domain"
	args={{
		value: -30,
		min: -100,
		max: 0
	}}
/>

<Story
	name="Custom class"
	args={{
		value: 4,
		class: 'h-4'
	}}
/>
