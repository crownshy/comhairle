<script lang="ts">
	import { replaceState } from '$app/navigation';
	import { page } from '$app/state';
	import * as Tabs from '$lib/components/ui/tabs';
	import { permissions } from '$lib/permissions.svelte';
	import MonitorStatsBar from './MonitorStatsBar.svelte';
	import FirstVisitHint from './FirstVisitHint.svelte';
	import QuestionCard from './QuestionCard.svelte';
	import TargetSparkline from './TargetSparkline.svelte';
	import ActionCalendar from './ActionCalendar.svelte';
	import ReachMini from './ReachMini.svelte';
	import StepFunnel from './StepFunnel.svelte';
	import RecruitmentTarget from './RecruitmentTarget.svelte';
	import SignupsOverTimeChart from './SignupsOverTimeChart.svelte';
	import RepresentationHint from './RepresentationHint.svelte';
	import ActionHint from './ActionHint.svelte';
	import { Button } from '$lib/components/ui/button';
	import RecruitmentChannels from './RecruitmentChannels.svelte';
	import RepresentationBars from './RepresentationBars.svelte';
	import RecentActivity from './RecentActivity.svelte';
	import ActivityHeatmap from './ActivityHeatmap.svelte';
	import TodayJourneys from './TodayJourneys.svelte';
	import FollowUpActions from './FollowUpActions.svelte';
	import * as sample from './sampleMonitorData';

	import type { PageProps } from './$types';

	// Prototype of the redesigned Monitor tab. All numbers come from ./sampleMonitorData.ts;
	// links point at the real sibling tabs. See README.md in this folder for the handoff notes.
	let { data }: PageProps = $props();

	const MONITOR_TABS = [
		{ value: 'glance', label: 'At a glance' },
		{ value: 'pulse', label: 'Live pulse' },
		{ value: 'journey', label: 'Journey' },
		{ value: 'recruitment', label: 'Recruitment' }
	] as const;
	type MonitorTab = (typeof MONITOR_TABS)[number]['value'];

	function isMonitorTab(value: string | null): value is MonitorTab {
		return MONITOR_TABS.some((tab) => tab.value === value);
	}

	// The active tab is local state, read once from `?tab=` so a refresh or a shared link
	// opens the same view. Switching only rewrites the URL (see writeSearchParam): a full
	// navigation would re-run the layout's data loading and make quick clicks get lost.
	const initialTab = page.url.searchParams.get('tab');
	let activeTab = $state<MonitorTab>(isMonitorTab(initialTab) ? initialTab : 'glance');

	/** Updates the address bar without a SvelteKit navigation (shallow routing). */
	function writeSearchParam(key: string, value: string) {
		const url = new URL(window.location.href);
		url.searchParams.set(key, value);
		// Only the query string of the current page changes, so there is nothing for `resolve()` to do.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		replaceState(url, page.state);
	}

	function showTab(value: string) {
		if (!isMonitorTab(value)) return;
		activeTab = value;
		writeSearchParam('tab', value);
	}

	let base = $derived(`/admin/conversations/${data.conversation.id}`);
	let canExport = $derived(
		permissions.can('conversation', 'conversation_export', data.conversation.id)
	);

	let showFunnelAs = $state<'people' | 'percent'>('people');

	// The Recruitment hint opens Demographic analysis on the demographic it names.
	let representationTab = $state(sample.SAMPLE_REPRESENTATION[0]?.name ?? '');
	/** Scrolls to a card on the page and moves focus to it. */
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
	// TODO(backend): load and save the target with the conversation. Here it lives in page state.
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

<Tabs.Root value={activeTab} onValueChange={showTab} class="flex flex-col gap-8 pb-16">
	<Tabs.List class="w-fit max-w-full" aria-label="Monitor sections">
		{#each MONITOR_TABS as tab (tab.value)}
			<Tabs.Trigger value={tab.value} class="px-4 text-base">{tab.label}</Tabs.Trigger>
		{/each}
	</Tabs.List>

	<!-- At a glance: verdict and the three questions -->
	<Tabs.Content value="glance" class="flex flex-col gap-12">
		<section aria-label="Overview" class="flex flex-col gap-6">
			<MonitorStatsBar
				variant="primary"
				caption="Day {sample.SAMPLE_DAY_OF_CONVERSATION} of {sample.SAMPLE_CONVERSATION_LENGTH_DAYS}"
				captionEnd="Updated 2 minutes ago"
				stats={[
					{
						name: 'Joined',
						amount: sample.SAMPLE_JOINED,
						unit: null,
						meaning: 'People who opened the conversation',
						explanation:
							'Counts everyone who opened the conversation at least once, through any link, email or QR code. People who were invited but never opened it are not counted.'
					},
					{
						name: 'Completed',
						amount: sample.SAMPLE_FINISHED,
						unit: null,
						meaning: `Completed every step. ${Math.round((sample.SAMPLE_FINISHED / sample.SAMPLE_JOINED) * 100)}% of everyone who joined`,
						explanation:
							'Counts people who reached the end of the last step. Someone who skipped an optional step still counts as completed.'
					},
					{
						name: 'Active now',
						amount: sample.SAMPLE_ACTIVE_NOW,
						unit: null,
						meaning: 'Taking part right now, in any step',
						explanation:
							'People who did something in the conversation in the last 5 minutes.'
					},
					{
						name: 'Time to complete',
						amount: sample.SAMPLE_MEDIAN_MINUTES_TO_FINISH,
						unit: 'min',
						meaning: 'Typical time from first step to last',
						explanation:
							'The median: half of the people who completed took less time than this, half took longer. Time away from the page is not counted.'
					}
				]}
			/>
			<FirstVisitHint tourHref="/admin/info/tools" />
			<div class="grid grid-cols-1 gap-5 lg:grid-cols-3">
				<QuestionCard
					eyebrow="Working"
					question="Are people completing?"
					value="{Math.round((sample.SAMPLE_FINISHED / sample.SAMPLE_JOINED) * 100)}%"
					unit="of joiners completed"
					trendLabel="Completion rate per step"
					bars={sample.SAMPLE_STEPS.map((step) => ({
						label: step.shortName,
						fullLabel: step.name,
						percent:
							step.reached === 0
								? 0
								: Math.round((step.finished / step.reached) * 100),
						flagged: step.name === worstStep.name
					}))}
					actionLabel="Review {worstStep.name}"
					actionHref="{base}/design"
					onShowDetails={() => showTab('journey')}
					detailsTab="Journey"
				/>
				<QuestionCard
					eyebrow="Recruitment"
					question="Are enough people joining?"
					value={String(sample.SAMPLE_JOINED)}
					unit="of {target} target"
					trendLabel="Running total of signups against the target"
					actionLabel="Open Recruit"
					actionHref="{base}/invites"
					onShowDetails={() => showTab('recruitment')}
					detailsTab="Recruitment"
				>
					{#snippet visual()}
						<TargetSparkline
							signups={sample.SAMPLE_SIGNUPS}
							{target}
							launchDate={sample.SAMPLE_LAUNCH_DATE}
							closingDate={sample.SAMPLE_CLOSING_DATE}
						/>
					{/snippet}
				</QuestionCard>
				<QuestionCard
					eyebrow="Reach"
					question="Are we hearing from everyone?"
					value="{reachDimension.rows.length - underRepresentedCount} of {reachDimension
						.rows.length}"
					unit="{reachDimension.groupNoun} well covered"
					trendLabel="Share of participants compared with the local population"
					actionLabel="Open Recruit"
					actionHref="{base}/invites"
					onShowDetails={() => showTab('recruitment')}
					detailsTab="Recruitment"
				>
					{#snippet visual()}
						<ReachMini
							dimensions={sample.SAMPLE_REPRESENTATION}
							bind:selected={reachDimensionName}
						/>
					{/snippet}
				</QuestionCard>
			</div>
			<ActionCalendar
				entries={sample.SAMPLE_ACTIVITY_LOG}
				launchDate={sample.SAMPLE_LAUNCH_DATE}
				closingDate={sample.SAMPLE_CLOSING_DATE}
				today={sample.SAMPLE_TODAY}
				conversationId={data.conversation.id}
			/>
		</section>
	</Tabs.Content>

	<!-- Pulse: what's happening right now, and who to follow up with -->
	<Tabs.Content value="pulse" class="flex flex-col gap-6">
		<ActionHint>
			Most people who stop leave at <strong>{worstStep.name}</strong>:
			{worstStep.reached - worstStep.finished} of the {worstStep.reached} who reached it didn't
			complete it.
			{#snippet actions()}
				<Button
					class="rounded-full bg-orange-700 text-white hover:bg-orange-800"
					onclick={() => scrollToCard('today-journeys')}>See more</Button
				>
			{/snippet}
		</ActionHint>
		<MonitorStatsBar
			variant="secondary"
			stats={[
				{
					name: 'Active now',
					amount: sample.SAMPLE_ACTIVE_NOW,
					unit: null,
					explanation:
						'People who did something in the conversation in the last 5 minutes, in any step.'
				},
				{
					name: 'Joined today',
					amount: 10,
					unit: null,
					explanation:
						'People who opened the conversation for the first time today, through any link, email or QR code. Counts reset at midnight.'
				},
				{
					name: 'Completed today',
					amount: 3,
					unit: null,
					explanation:
						'People who reached the end of the last step today, whenever they first joined. Counts reset at midnight.'
				}
			]}
		/>
		<TodayJourneys steps={sample.SAMPLE_STEPS} journeys={sample.SAMPLE_TODAY_JOURNEYS} />
		<ActivityHeatmap
			days={sample.HEATMAP_DAYS}
			slots={sample.HEATMAP_SLOTS}
			counts={sample.SAMPLE_HEATMAP}
		/>

		<div class="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]">
			<RecentActivity activity={sample.SAMPLE_ACTIVITY} />
			<FollowUpActions
				actions={[
					{
						count: worstStep.reached - worstStep.finished,
						who: `stuck at ${worstStep.name}`,
						why: "Started but haven't been back for 3+ days.",
						label: 'Send a reminder',
						href: `${base}/notifications`,
						kind: 'message'
					},
					{
						count: sample.SAMPLE_FINISHED,
						who: 'completed',
						why: 'Thank them, and let them know when the report is out.',
						label: 'Send a thank-you',
						href: `${base}/notifications`,
						kind: 'message'
					},

					{
						count: sample.USED_THINKING_SPACE,
						who: 'download "Thinking Space" summary',
						why: 'Get a sense of how people think.',
						label: 'Download summaries',
						href: `/api/conversation/${data.conversation.id}/demographics/export`,
						kind: 'download',
						disabled: !canExport,
						primary: true
					}
				]}
			/>
		</div>
	</Tabs.Content>

	<!-- Journey: where people get stuck -->
	<Tabs.Content value="journey" class="flex flex-col gap-6">
		<ActionHint tone="info">
			{worstStep.inactiveThreeDays} people started <strong>{worstStep.name}</strong> but
			haven't been back for 3 days or more. A short reminder can bring them back.
			{#snippet actions()}
				<Button href="{base}/notifications" class="rounded-full">Message them</Button>
			{/snippet}
		</ActionHint>
		<StepFunnel
			steps={sample.SAMPLE_STEPS}
			joined={sample.SAMPLE_JOINED}
			worstStep={worstStep.name}
			bind:showAs={showFunnelAs}
		/>
	</Tabs.Content>

	<!-- Recruitment: who is joining, from where, and whether they reflect the population -->
	<Tabs.Content value="recruitment" class="flex flex-col gap-12">
		<section aria-labelledby="recruitment-heading" class="flex flex-col gap-6">
			<RepresentationHint
				dimensions={sample.SAMPLE_REPRESENTATION}
				onView={viewRepresentation}
			/>
			<RecruitmentTarget
				joined={sample.SAMPLE_JOINED}
				completed={sample.SAMPLE_FINISHED}
				{target}
				{evenPaceToday}
				closingDate={sample.SAMPLE_CLOSING_DATE}
				daysLeft={sample.SAMPLE_CONVERSATION_LENGTH_DAYS -
					sample.SAMPLE_DAY_OF_CONVERSATION}
				onTargetChange={(value) => (target = value)}
			/>
			<RepresentationBars
				bind:selected={representationTab}
				dimensions={sample.SAMPLE_REPRESENTATION}
			/>
			<SignupsOverTimeChart
				signups={sample.SAMPLE_SIGNUPS}
				completions={sample.SAMPLE_COMPLETIONS}
				{targetPace}
				projection={sample.SAMPLE_PROJECTION}
				actions={sample.SAMPLE_ACTIVITY_LOG}
				{target}
			/>
			<RecruitmentChannels
				channels={sample.SAMPLE_CHANNELS}
				conversationId={data.conversation.id}
			/>
		</section>
	</Tabs.Content>
</Tabs.Root>
