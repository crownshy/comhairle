<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { permissions } from '$lib/permissions.svelte';
	import FirstVisitHint from './FirstVisitHint.svelte';
	import ParticipationFlow from './ParticipationFlow.svelte';
	import QuestionCard from './QuestionCard.svelte';
	import FollowUpActions from './FollowUpActions.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { monitorTour } from './tour';
	import { type MonitorRoute } from './sub-routes';
	import type { PageProps } from './$types';
	import type { WorkflowStats } from '@crownshy/api-client/api';
	// import TargetSparkline from '../TargetSparkline.svelte';
	// import ReachMini from '../ReachMini.svelte';
	// import * as sample from '../sampleMonitorData';

	useTour(() => monitorTour);

	let { data }: PageProps = $props();

	let workflowStats: WorkflowStats | null = $derived(data.workflowStats);

	let stepsWithStats = $derived(
		data.workflowSteps?.map((step) => {
			const matchingStats = workflowStats?.stepStats.find((stat) => stat.id === step.id);
			const completed = matchingStats?.completed ?? 0;
			const reached = completed + (matchingStats?.inProgress ?? 0);
			return {
				id: step.id,
				name: step.name,
				reached,
				completed
			};
		}) ?? []
	);

	let worstStep = $derived(
		stepsWithStats.reduce(
			(worst, step) =>
				step.reached - step.completed > worst.reached - worst.completed ? step : worst,
			0
		)
	);

	let medianActiveSecs = $derived(
		workflowStats?.stepStats.reduce((acc, step) => acc + (step.medianActiveSeconds ?? 0), 0) ??
			0
	);
	let medianTimeLabel = $derived.by(() => {
		if (medianActiveSecs < 60) {
			return `${medianActiveSecs} sec`;
		} else {
			return `${Math.round(medianActiveSecs / 60)}`;
		}
	});

	async function showTab(route: MonitorRoute) {
		await goto(
			resolve(route, {
				conversation_id: data.conversationId
			})
		);
	}

	let base = $derived(`/admin/conversations/${data.conversation.id}`);
	let canExport = $derived(
		permissions.can('conversation', 'conversation_export', data.conversation.id)
	);

	// The Recruitment hint opens Demographic analysis on the demographic it names.
	// let representationTab = $state(sample.SAMPLE_REPRESENTATION[0]?.name ?? '');
	// function scrollToCard(id: string) {
	// 	const card = document.getElementById(id);
	// 	const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
	// 	card?.scrollIntoView({ behavior: reduceMotion ? 'auto' : 'smooth', block: 'start' });
	// 	card?.focus({ preventScroll: true });
	// }
	//
	// function viewRepresentation(dimensionName: string) {
	// 	representationTab = dimensionName;
	// 	scrollToCard('demographic-analysis');
	// }

	// TODO(backend): load and save the target with the conversation (page state only for now).
	// let target = $state(sample.SAMPLE_TARGET);
	// let targetPace = $derived([
	// 	sample.SAMPLE_TARGET_PACE[0],
	// 	{ day: sample.SAMPLE_CLOSING_DATE, total: target }
	// ]);
	// let evenPaceToday = $derived(
	// 	Math.round(
	// 		(target * sample.SAMPLE_DAY_OF_CONVERSATION) / sample.SAMPLE_CONVERSATION_LENGTH_DAYS
	// 	)
	// );
	// let closingLabel = $derived(
	// 	sample.SAMPLE_CLOSING_DATE.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' })
	// );
	// // At a glance Reach card: which demographic is selected.
	// let reachDimensionName = $state(sample.SAMPLE_REPRESENTATION[0].name);
	// let reachDimension = $derived(
	// 	sample.SAMPLE_REPRESENTATION.find((dimension) => dimension.name === reachDimensionName) ??
	// 		sample.SAMPLE_REPRESENTATION[0]
	// );
	// let underRepresentedCount = $derived(
	// 	reachDimension.rows.filter((row) => sample.isUnderRepresented(row)).length
	// );
</script>

<svelte:head>
	<title>Monitor prototype - Comhairle Admin</title>
</svelte:head>

{#if workflowStats}
	<div class="flex flex-col gap-12">
		<section aria-label="Overview" class="flex flex-col gap-6">
			<FirstVisitHint tourHref="/admin/info/tools" />
			<!-- TODO: removed from ParticipationFlow component below as closing date not currently supported -->
			<!-- caption="Day {sample.SAMPLE_DAY_OF_CONVERSATION} of {sample.SAMPLE_CONVERSATION_LENGTH_DAYS} · closes {closingLabel}" -->
			<ParticipationFlow
				recruited={workflowStats.recruitedUsers}
				joined={workflowStats.totalUsers}
				completed={workflowStats.finishedUsers}
			/>
			<div class="flex gap-5">
				<QuestionCard
					eyebrow="Working"
					question="Are people completing?"
					value={workflowStats.totalUsers > 0
						? `${Math.round(
								(workflowStats.finishedUsers / workflowStats.totalUsers) * 100
							)}%`
						: ''}
					unit={workflowStats.totalUsers > 0 ? 'of joiners completed' : ''}
					extraValue={workflowStats.totalUsers > 0 ? medianTimeLabel : ''}
					extraUnit="typical time to complete"
					trendLabel="Completion rate per step"
					bars={stepsWithStats.map((step) => ({
						id: step.id,
						label: step.name,
						fullLabel: step.name,
						percent:
							step.reached === 0
								? 0
								: Math.round((step.completed / step.reached) * 100),
						flagged: step.name === worstStep.name
					}))}
					actionLabel="Review {worstStep.name}"
					actionHref="{base}/design"
					onShowDetails={() =>
						showTab('/(admin)/admin/conversations/[conversation_id]/monitor/journey')}
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
{:else}
	<h2>Could not collect statistics for this conversation at this time.</h2>
{/if}
