<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import AreaOfConsensus from '$lib/reports/polis/AreaOfConsensus.svelte';
	import type { PolisReportData } from '$lib/tools/polis/reportTypes';
	import { getConsensusStatements, getDifferenceStatements } from '$lib/tools/polis/report';
	import reportData from '$lib/types/report-data.json';

	// Real captured Polis conversation (local housing). The consensus and difference
	// lists are ranked by the same helpers PolisInsights feeds in, so the story shows
	// the component with the data it renders in the app.
	const report = reportData as PolisReportData;
	const groups = report.groups;
	const consensus = getConsensusStatements(report);
	const difference = getDifferenceStatements(report);

	const { Story } = defineMeta({
		title: 'Components/reports/AreaOfConsensus',
		component: AreaOfConsensus,
		tags: ['autodocs'],
		argTypes: {
			title: { control: 'text' },
			comments: { control: false },
			groups: { control: false },
			onDownloadCsv: { control: false }
		}
	});
</script>

<!-- The consensus section: statements every group agrees (or disagrees) on. Starts
     collapsed to 4 rows; the seed/participant chips and "See all" expander are local
     view state, so they work live here. -->
<Story
	name="Area of consensus"
	args={{ title: 'Area of consensus', comments: consensus, groups, onDownloadCsv: fn() }}
>
	{#snippet children(args)}
		<div class="w-full max-w-3xl">
			<AreaOfConsensus {...args} />
		</div>
	{/snippet}
</Story>

<!-- Same component, difference list: the `title` prop decides which section it is. -->
<Story
	name="Area of disagreement"
	args={{ title: 'Area of disagreement', comments: difference, groups, onDownloadCsv: fn() }}
>
	{#snippet children(args)}
		<div class="w-full max-w-3xl">
			<AreaOfConsensus {...args} />
		</div>
	{/snippet}
</Story>

<!-- CSV handler omitted, so the Download button is hidden. -->
<Story name="Without download" args={{ title: 'Area of consensus', comments: consensus, groups }}>
	{#snippet children(args)}
		<div class="w-full max-w-3xl">
			<AreaOfConsensus {...args} />
		</div>
	{/snippet}
</Story>

<!-- Empty state: no statements match, so the list is replaced by the empty message. -->
<Story name="Empty" args={{ title: 'Area of consensus', comments: [], groups }}>
	{#snippet children(args)}
		<div class="w-full max-w-3xl">
			<AreaOfConsensus {...args} />
		</div>
	{/snippet}
</Story>
