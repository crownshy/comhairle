<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import OpinionGroups from '$lib/reports/polis/OpinionGroups.svelte';
	import type { PolisReportData } from '$lib/tools/polis/reportTypes';
	import reportData from '$lib/types/report-data.json';

	// Real captured Polis conversation (local housing), the same fixture the other
	// Polis report stories use. Its two groups each carry representative_comments, so
	// the section renders both cards with live vote blocks.
	const report = reportData as PolisReportData;

	const { Story } = defineMeta({
		title: 'Components/reports/OpinionGroups',
		component: OpinionGroups,
		argTypes: {
			comments: { control: false },
			groups: { control: false }
		}
	});

	// A single group, to show the section handling the degenerate case.
	const oneGroup = report.groups.slice(0, 1);
	// No groups at all -> the chip row and card list are empty.
	const noGroups: PolisReportData['groups'] = [];
</script>

<!-- Default: both opinion groups. The AI name + summary are hidden here (the live
     report has no source for them yet -- see OpinionGroupCard for the generated
     version); the size line, membership share, and representative statements are real.
     The nav chips focus a single group; the per-card "See all" expander is local view
     state, so both work live here. -->
<Story name="Default" args={{ comments: report.comments, groups: report.groups }} />

<!-- One group only: the chip row has a single chip and one card renders. -->
<Story name="Single group" args={{ comments: report.comments, groups: oneGroup }} />

<!-- No groups: heading only, no chips or cards. -->
<Story name="No groups" args={{ comments: report.comments, groups: noGroups }} />
