<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ConsensusContinuum from '$lib/reports/polis/ConsensusContinuum.svelte';
	import { sampleReportData } from './polisReport.fixture';

	const { Story } = defineMeta({
		title: 'Components/reports/ConsensusContinuum',
		component: ConsensusContinuum
	});

	// Strip divisiveness so the beeswarm has nothing to place -> empty state. The
	// component filters to scored comments itself, so Default just passes the fixture.
	const unscored = sampleReportData.comments.map((c) => ({ ...c, divisiveness: null }));
</script>

<!-- Default: the captured 5-statement report, most-divisive statement pre-selected. -->
<Story
	name="Default"
	args={{ comments: sampleReportData.comments, groups: sampleReportData.groups }}
/>

<!-- No statement has a divisiveness score yet (too few votes) -> empty state. -->
<Story name="NoScores" args={{ comments: unscored, groups: sampleReportData.groups }} />
