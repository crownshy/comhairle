<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import VoteBar from '$lib/reports/polis/VoteBar.svelte';

	const { Story } = defineMeta({
		title: 'Components/reports/VoteBar',
		component: VoteBar,
		tags: ['autodocs'],
		argTypes: {
			label: { control: 'text' },
			agreed: { control: { type: 'range', min: 0, max: 100 } },
			disagreed: { control: { type: 'range', min: 0, max: 100 } },
			passed: { control: { type: 'range', min: 0, max: 100 } },
			notVoted: { control: { type: 'range', min: 0, max: 100 } }
		}
	});
</script>

<!-- Hover (or focus) a segment to see its category and percentage. -->
<Story
	name="Default"
	args={{ label: 'OVERALL', agreed: 52, disagreed: 21, passed: 9, notVoted: 18 }}
>
	{#snippet children(args)}
		<div class="w-full max-w-xl">
			<VoteBar {...args} />
		</div>
	{/snippet}
</Story>

<Story name="No label" args={{ agreed: 64, disagreed: 28, passed: 4, notVoted: 4 }}>
	{#snippet children(args)}
		<div class="w-full max-w-xl">
			<VoteBar {...args} />
		</div>
	{/snippet}
</Story>

<Story
	name="Mostly not voted"
	args={{ label: 'GROUP B', agreed: 18, disagreed: 12, passed: 6, notVoted: 64 }}
>
	{#snippet children(args)}
		<div class="w-full max-w-xl">
			<VoteBar {...args} />
		</div>
	{/snippet}
</Story>
