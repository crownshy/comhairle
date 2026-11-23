<script lang="ts">
	import { replaceState, goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { permissions } from '$lib/permissions.svelte';
	import FirstVisitHint from '../FirstVisitHint.svelte';
	import ParticipationFlow from '../ParticipationFlow.svelte';
	import QuestionCard from '../QuestionCard.svelte';
	import TargetSparkline from '../TargetSparkline.svelte';
	import ReachMini from '../ReachMini.svelte';
	import { Button } from '$lib/components/ui/button';
	import FollowUpActions from '../FollowUpActions.svelte';
	import * as sample from '../sampleMonitorData';

	import type { PageProps } from './$types';

	// Prototype: all numbers come from ./sampleMonitorData.ts. See README.md for handoff notes.
	let { data }: PageProps = $props();

	type MonitorTab = 'overview' | 'pulse' | 'journey' | 'recruitment';
	async function showTab(value: MonitorTab) {
		await goto(
			resolve(`/(admin)/admin/conversations/[conversation_id]/monitor/prototype/${value}`, {
				conversation_id: data.conversationId
			})
		);
	}

	let base = $derived(`/admin/conversations/${data.conversation.id}`);
	let canExport = $derived(
		permissions.can('conversation', 'conversation_export', data.conversation.id)
	);

	// The Recruitment hint opens Demographic analysis on the demographic it names.
	let representationTab = $state(sample.SAMPLE_REPRESENTATION[0]?.name ?? '');
	function scrollToCard(id: string) {
		const card = document.getElementById(id);
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		card?.scrollIntoView({ behavior: reduceMotion ? 'auto' : 'smooth', block: 'start' });
		card?.focus({ preventScroll: true });
	}

	function viewRepresentation(dimensionName: string) {
		representationTab = dimensionName;
		scrollToCard('demographic-analysis');
	}

	let worstStep = $derived(
		sample.SAMPLE_STEPS.reduce((worst, step) =>
			step.reached - step.finished > worst.reached - worst.finished ? step : worst
		)
	);
	// TODO(backend): load and save the target with the conversation (page state only for now).
	let target = $state(sample.SAMPLE_TARGET);
	let targetPace = $derived([
		sample.SAMPLE_TARGET_PACE[0],
		{ day: sample.SAMPLE_CLOSING_DATE, total: target }
	]);
	let evenPaceToday = $derived(
		Math.round(
			(target * sample.SAMPLE_DAY_OF_CONVERSATION) / sample.SAMPLE_CONVERSATION_LENGTH_DAYS
		)
	);
	let closingLabel = $derived(
		sample.SAMPLE_CLOSING_DATE.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' })
	);
	// At a glance Reach card: which demographic is selected.
	let reachDimensionName = $state(sample.SAMPLE_REPRESENTATION[0].name);
	let reachDimension = $derived(
		sample.SAMPLE_REPRESENTATION.find((dimension) => dimension.name === reachDimensionName) ??
			sample.SAMPLE_REPRESENTATION[0]
	);
	let underRepresentedCount = $derived(
		reachDimension.rows.filter((row) => sample.isUnderRepresented(row)).length
	);
</script>

<svelte:head>
	<title>Monitor prototype - Comhairle Admin</title>
</svelte:head>

<!-- At a glance -->
<div class="flex flex-col gap-12">
	<section aria-label="Overview" class="flex flex-col gap-6">
		<FirstVisitHint tourHref="/admin/info/tools" />
		<!-- TODO: removed from ParticipationFlow component below as closing date not currently supported -->
		<!-- caption="Day {sample.SAMPLE_DAY_OF_CONVERSATION} of {sample.SAMPLE_CONVERSATION_LENGTH_DAYS} · closes {closingLabel}" -->
		<ParticipationFlow
			recruited={sample.SAMPLE_RECRUITED}
			joined={sample.SAMPLE_JOINED}
			completed={sample.SAMPLE_FINISHED}
		/>
		<div class="flex gap-5">
			<QuestionCard
				eyebrow="Working"
				question="Are people completing?"
				value="{Math.round((sample.SAMPLE_FINISHED / sample.SAMPLE_JOINED) * 100)}%"
				unit="of joiners completed"
				extraValue="{sample.SAMPLE_MEDIAN_MINUTES_TO_FINISH} min"
				extraUnit="typical time to complete"
				trendLabel="Completion rate per step"
				bars={sample.SAMPLE_STEPS.map((step) => ({
					label: step.shortName,
					fullLabel: step.name,
					percent:
						step.reached === 0 ? 0 : Math.round((step.finished / step.reached) * 100),
					flagged: step.name === worstStep.name
				}))}
				actionLabel="Review {worstStep.name}"
				actionHref="{base}/design"
				onShowDetails={() => showTab('journey')}
				detailsTab="Journey"
			/>
			<!-- TODO: commented out as dependent on recruitment target and closing date -->
			<!-- <QuestionCard -->
			<!-- 	eyebrow="Recruitment" -->
			<!-- 	question="Are enough people joining?" -->
			<!-- 	value={String(sample.SAMPLE_JOINED)} -->
			<!-- 	unit="of {target} target" -->
			<!-- 	trendLabel="Running total of signups against the target" -->
			<!-- 	actionLabel="Open Recruit" -->
			<!-- 	actionHref="{base}/invites" -->
			<!-- 	onShowDetails={() => showTab('recruitment')} -->
			<!-- 	detailsTab="Recruitment" -->
			<!-- > -->
			<!-- 	{#snippet visual()} -->
			<!-- 		<TargetSparkline -->
			<!-- 			signups={sample.SAMPLE_SIGNUPS} -->
			<!-- 			{target} -->
			<!-- 			launchDate={sample.SAMPLE_LAUNCH_DATE} -->
			<!-- 			closingDate={sample.SAMPLE_CLOSING_DATE} -->
			<!-- 		/> -->
			<!-- 	{/snippet} -->
			<!-- </QuestionCard> -->
			<!-- <QuestionCard -->
			<!-- 	eyebrow="Reach" -->
			<!-- 	question="Are we hearing from everyone?" -->
			<!-- 	value="{reachDimension.rows.length - underRepresentedCount} of {reachDimension -->
			<!-- 		.rows.length}" -->
			<!-- 	unit="{reachDimension.groupNoun} well covered" -->
			<!-- 	trendLabel="Share of participants compared with the local population" -->
			<!-- 	actionLabel="Open Recruit" -->
			<!-- 	actionHref="{base}/invites" -->
			<!-- 	onShowDetails={() => showTab('recruitment')} -->
			<!-- 	detailsTab="Recruitment" -->
			<!-- > -->
			<!-- 	{#snippet visual()} -->
			<!-- 		<ReachMini -->
			<!-- 			dimensions={sample.SAMPLE_REPRESENTATION} -->
			<!-- 			bind:selected={reachDimensionName} -->
			<!-- 		/> -->
			<!-- 	{/snippet} -->
			<!-- </QuestionCard> -->
		</div>
	</section>
	<FollowUpActions
		exports={[
			// TODO: commented out as dependent on data export for each tool type
			// {
			// 	count: sample.USED_THINKING_SPACE,
			// 	title: 'Thinking space summaries',
			// 	description: 'Get a sense of how people think.',
			// 	label: 'Download summaries',
			// 	// TODO(backend): no summaries export yet; this points at the demographics export.
			// 	href: `/api/conversation/${data.conversation.id}/demographics/export`,
			// 	disabled: !canExport,
			// 	primary: true
			// },
			{
				title: 'Contacts',
				description:
					'A list of users who have opted in to being contacted on this engagement.',
				label: 'Download contacts',
				href: `/api/conversation/${data.conversation.id}/contacts/export`,
				disabled: !canExport
			},
			{
				title: 'Demographics',
				description:
					'Demographic data from user profiles for participants in this conversation.',
				label: 'Download demographics',
				href: `/api/conversation/${data.conversation.id}/demographics/export`,
				disabled: !canExport
			}
		]}
	/>
</div>
