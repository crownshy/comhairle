<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import OpinionGroupCard from '$lib/reports/polis/OpinionGroupCard.svelte';
	import type { PolisReportData } from '$lib/tools/polis/reportTypes';
	import reportData from '$lib/types/report-data.json';

	// Real captured Polis conversation (local housing). One group's card, with its
	// representative_comments resolved against the full comment list for the bars.
	const report = reportData as PolisReportData;
	const group = report.groups[0];

	// Placeholder AI copy -- the payload carries no group names or summaries yet.
	const placeholder = {
		name: 'Progressive Digital Advocates',
		summary:
			'This group strongly believes in the constructive role technology plays in young ' +
			"people's development. They champion digital literacy education and tend to support " +
			'platform-driven innovations that empower rather than restrict youth.'
	};

	// Same group with no representative comments -> the "no statements yet" message.
	const groupNoStatements = { ...group, representative_comments: [] };

	const { Story } = defineMeta({
		title: 'Components/reports/OpinionGroupCard',
		component: OpinionGroupCard,
		argTypes: {
			group: { control: false },
			comments: { control: false },
			groups: { control: false },
			placeholder: { control: false }
		}
	});
</script>

<!-- Default: size line, placeholder name/summary behind the "AI Generated" badge,
     then the group's representative statements (starts collapsed to 3, "See all N"
     expander is local view state). -->
<Story
	name="Default"
	args={{ group, comments: report.comments, groups: report.groups, placeholder }}
/>

<!-- No representative comments: the statement list is replaced by the empty message. -->
<Story
	name="No statements"
	args={{
		group: groupNoStatements,
		comments: report.comments,
		groups: report.groups,
		placeholder
	}}
/>
