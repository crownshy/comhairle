<script lang="ts">
	import RecruitmentTarget from '../RecruitmentTarget.svelte';
	import RepresentationBars from '../RepresentationBars.svelte';
	import SignupsOverTimeChart from '../SignupsOverTimeChart.svelte';
	import RecruitmentChannels from '../RecruitmentChannels.svelte';
	import RepresentationHint from '../RepresentationHint.svelte';
	import * as sample from '../sampleMonitorData';

	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<!-- Recruitment -->
<div class="flex flex-col gap-12">
	<section aria-labelledby="recruitment-heading" class="flex flex-col gap-6">
		<!-- TODO: commented out as dependent on demographics baseline figures -->
		<!-- <RepresentationHint -->
		<!-- 	dimensions={sample.SAMPLE_REPRESENTATION} -->
		<!-- 	onView={viewRepresentation} -->
		<!-- /> -->
		<!-- <RecruitmentTarget -->
		<!-- 	joined={sample.SAMPLE_JOINED} -->
		<!-- 	completed={sample.SAMPLE_FINISHED} -->
		<!-- 	{target} -->
		<!-- 	{evenPaceToday} -->
		<!-- 	closingDate={sample.SAMPLE_CLOSING_DATE} -->
		<!-- 	daysLeft={sample.SAMPLE_CONVERSATION_LENGTH_DAYS - -->
		<!-- 		sample.SAMPLE_DAY_OF_CONVERSATION} -->
		<!-- 	onTargetChange={(value) => (target = value)} -->
		<!-- /> -->
		<!-- <RepresentationBars -->
		<!-- 	bind:selected={representationTab} -->
		<!-- 	dimensions={sample.SAMPLE_REPRESENTATION} -->
		<!-- /> -->
		<!-- <SignupsOverTimeChart -->
		<!-- 	signups={sample.SAMPLE_SIGNUPS} -->
		<!-- 	completions={sample.SAMPLE_COMPLETIONS} -->
		<!-- 	{targetPace} -->
		<!-- 	projection={sample.SAMPLE_PROJECTION} -->
		<!-- 	actions={sample.SAMPLE_ACTIVITY_LOG} -->
		<!-- 	{target} -->
		<!-- /> -->
		<RecruitmentChannels
			channels={sample.SAMPLE_CHANNELS}
			conversationId={data.conversationId}
		/>
	</section>
</div>
