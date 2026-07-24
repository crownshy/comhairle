<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StatementVoteBlock from '$lib/reports/polis/StatementVoteBlock.svelte';
	import type { PolisReportData } from '$lib/tools/polis/reportTypes';
	import { getConsensusStatements, getDifferenceStatements } from '$lib/tools/polis/report';
	import reportData from '$lib/types/report-data.json';

	// Real captured Polis conversation (local housing). Deriving the sample
	// statements via the same helpers the app uses keeps the story honest as the
	// report data shape evolves, rather than hand-authoring vote counts.
	const report = reportData as PolisReportData;
	const groups = report.groups;
	const consensus = getConsensusStatements(report);
	const difference = getDifferenceStatements(report);

	const { Story } = defineMeta({
		title: 'Components/reports/StatementVoteBlock',
		component: StatementVoteBlock,
		tags: ['autodocs'],
		argTypes: {
			comment: { control: false },
			groups: { control: false }
		}
	});
</script>

<!-- A consensus statement: every group lands on the same side, so the OVERALL and
     each group bar lean the same way. -->
<Story name="Consensus statement" args={{ comment: consensus[0], groups }}>
	{#snippet children(args)}
		<div class="w-full max-w-2xl">
			<StatementVoteBlock {...args} />
		</div>
	{/snippet}
</Story>

<!-- A divisive statement: the groups split, so the group bars diverge from OVERALL
     and from each other. -->
<Story name="Divisive statement" args={{ comment: difference[0], groups }}>
	{#snippet children(args)}
		<div class="w-full max-w-2xl">
			<StatementVoteBlock {...args} />
		</div>
	{/snippet}
</Story>
