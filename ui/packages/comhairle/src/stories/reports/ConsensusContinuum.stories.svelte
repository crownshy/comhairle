<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ConsensusContinuum from '$lib/reports/polis/ConsensusContinuum.svelte';
	import type { PolisReportData } from '$lib/tools/polis/reportTypes';
	import reportData from '$lib/types/report-data.json';

	// Real captured Polis conversation (local housing), the same fixture the other
	// Polis report stories use.
	const report = reportData as PolisReportData;

	const { Story } = defineMeta({
		title: 'Components/reports/ConsensusContinuum',
		component: ConsensusContinuum
	});

	// Strip divisiveness so the beeswarm has nothing to place -> empty state. The
	// component filters to scored comments itself, so Default just passes the fixture.
	const unscored = report.comments.map((c) => ({ ...c, divisiveness: null }));
</script>

<!-- Default: the captured report, most-divisive statement pre-selected. -->
<Story name="Default" args={{ comments: report.comments, groups: report.groups }} />

<!-- No statement has a divisiveness score yet (too few votes) -> empty state. -->
<Story name="NoScores" args={{ comments: unscored, groups: report.groups }} />
