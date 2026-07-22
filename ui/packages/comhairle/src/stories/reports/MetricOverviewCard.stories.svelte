<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import MetricOverviewCard from '$lib/reports/MetricOverviewCard.svelte';

	const { Story } = defineMeta({
		title: 'Components/reports/MetricOverviewCard',
		component: MetricOverviewCard,
		tags: ['autodocs'],
		argTypes: {
			superText: { control: 'text' },
			metric: { control: 'number' },
			subText: { control: 'text' }
		}
	});
</script>

<Story name="Default" args={{ superText: 'Participants', metric: 18, subText: 'curious minds' }}>
	{#snippet children(args)}
		<div class="w-full max-w-sm">
			<MetricOverviewCard {...args} />
		</div>
	{/snippet}
</Story>
