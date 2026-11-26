<script lang="ts">
	import RecruitmentChannels from '../RecruitmentChannels.svelte';
	// import RecruitmentTarget from '../RecruitmentTarget.svelte';
	// import RepresentationBars from '../RepresentationBars.svelte';
	// import SignupsOverTimeChart from '../SignupsOverTimeChart.svelte';
	// import RepresentationHint from '../RepresentationHint.svelte';
	// import * as sample from '../sampleMonitorData';
	import type { RecruitmentChannel } from '../sampleMonitorData';

	import type { PageProps } from './$types';

	let { data, params }: PageProps = $props();

	const emailInvites = $derived(
		data.invites.filter(
			(invite) => typeof invite.inviteType !== 'string' && 'email' in invite.inviteType
		)
	);
	const otherInvites = $derived(
		data.invites.filter((invite) => typeof invite.inviteType === 'string')
	);

	let recruitmentChannels = $derived.by(() => {
		const acceptedEmailInvites = emailInvites.filter(
			(invite) => invite.acceptCount === 1
		).length;
		const emailChannel: RecruitmentChannel = {
			type: 'Email',
			name: `Invite list (${emailInvites.length} sent)`,
			joined: acceptedEmailInvites,
			conversion: `${((acceptedEmailInvites / emailInvites.length) * 100).toFixed(2)}% of invited`
		};

		const nonEmailChannels = otherInvites.map((invite) => {
			let inviteType: string = '';
			if (typeof invite.inviteType === 'string') inviteType = invite.inviteType;

			return {
				type: inviteType,
				name: invite.label || 'Unnamed',
				joined: invite.acceptCount,
				conversion: 'No conversion data'
			};
		});

		return [emailChannel, ...nonEmailChannels];
	});
</script>

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
			channels={recruitmentChannels}
			conversationId={params.conversation_id}
		/>
	</section>
</div>
