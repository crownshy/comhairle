<script lang="ts">
	import { replaceState } from '$app/navigation';
	import { page } from '$app/state';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import * as Tabs from '$lib/components/ui/tabs';
	import { cn } from '$lib/utils';
	import { permissions } from '$lib/permissions.svelte';
	import AiInsight from './AiInsight.svelte';
	import MonitorStatsBar from './MonitorStatsBar.svelte';
	import MonitorTimeline from './MonitorTimeline.svelte';
	import FirstVisitHint from './FirstVisitHint.svelte';
	import MonitorSummary from './MonitorSummary.svelte';
	import QuestionCard from './QuestionCard.svelte';
	import TargetSparkline from './TargetSparkline.svelte';
	import ReachMini from './ReachMini.svelte';
	import StepFunnel from './StepFunnel.svelte';
	import DropOffCallout from './DropOffCallout.svelte';
	import RecruitmentTarget from './RecruitmentTarget.svelte';
	import SignupsOverTimeChart from './SignupsOverTimeChart.svelte';
	import RecruitmentChannels from './RecruitmentChannels.svelte';
	import RepresentationBars from './RepresentationBars.svelte';
	import LivePulse from './LivePulse.svelte';
	import ActivityHeatmap from './ActivityHeatmap.svelte';
	import TodayJourneys from './TodayJourneys.svelte';
	import FollowUpActions from './FollowUpActions.svelte';
	import * as sample from './sampleMonitorData';

	import type { PageProps } from './$types';

	// Design prototype for the Monitor tab. Every number on this page is sample data from
	// ./sampleMonitorData.ts; links point at the real sibling tabs.
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

	// Simple shows only what is counted or calculated from data. AI adds model-written
	// interpretation, each piece labelled "AI summary". Cool is Simple with the experimental
	// timeline header on At a glance. Lives in `?mode=` like the tab.
	const MODES = [
		{ value: 'simple', label: 'Simple' },
		{ value: 'ai', label: 'AI' },
		{ value: 'cool', label: 'Cool' }
	] as const;
	type Mode = (typeof MODES)[number]['value'];
	const initialMode = page.url.searchParams.get('mode');
	let mode = $state<Mode>(
		MODES.find((option) => option.value === initialMode)?.value ?? 'simple'
	);
	let isAi = $derived(mode === 'ai');

	// Cool mode only: preview the running timeline or the day-one empty state.
	let timelinePreview = $state<'running' | 'dayOne'>('running');

	/** Updates the address bar without a SvelteKit navigation (shallow routing). */
	function writeSearchParam(key: string, value: string) {
		const url = new URL(window.location.href);
		url.searchParams.set(key, value);
		replaceState(url, page.state);
	}

	function showTab(value: string) {
		if (!isMonitorTab(value)) return;
		activeTab = value;
		writeSearchParam('tab', value);
	}

	function setMode(value: Mode) {
		mode = value;
		writeSearchParam('mode', value);
	}

	let base = $derived(`/admin/conversations/${data.conversation.id}`);
	let canExport = $derived(
		permissions.can('conversation', 'conversation_export', data.conversation.id)
	);

	let showFunnelAs = $state<'people' | 'percent'>('people');

	let worstStep = $derived(
		sample.SAMPLE_STEPS.reduce((worst, step) =>
			step.reached - step.finished > worst.reached - worst.finished ? step : worst
		)
	);
	let typicalMinutes = $derived.by(() => {
		const others = sample.SAMPLE_STEPS.filter((step) => step.name !== worstStep.name);
		const sorted = others.map((step) => step.medianMinutes).sort((a, b) => a - b);
		return sorted[Math.floor(sorted.length / 2)];
	});
	let evenPaceToday = $derived(
		Math.round(
			(sample.SAMPLE_TARGET * sample.SAMPLE_DAY_OF_CONVERSATION) /
				sample.SAMPLE_CONVERSATION_LENGTH_DAYS
		)
	);
	// Reach card: which demographic the admin is looking at.
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

<PageHeader
	title="Monitor"
	description="See how your conversation is working, monitor recruitment and user progress and check out your reach"
>
	{#snippet actions()}
		<div class="flex flex-wrap items-center gap-3">
			<div
				role="group"
				aria-label="Summary mode"
				class="bg-card inline-flex rounded-full border p-1"
			>
				{#each MODES as option (option.value)}
					<button
						type="button"
						aria-pressed={mode === option.value}
						onclick={() => setMode(option.value)}
						class={cn(
							'min-h-9 rounded-full px-4 text-sm font-medium',
							mode === option.value
								? 'bg-sidebar text-sidebar-foreground'
								: 'text-muted-foreground'
						)}
					>
						{option.label}
					</button>
				{/each}
			</div>
		</div>
	{/snippet}
</PageHeader>

<Tabs.Root value={activeTab} onValueChange={showTab} class="flex flex-col gap-8 pb-16">
	<Tabs.List class="w-fit max-w-full" aria-label="Monitor sections">
		{#each MONITOR_TABS as tab (tab.value)}
			<Tabs.Trigger value={tab.value} class="px-4 text-base">{tab.label}</Tabs.Trigger>
		{/each}
	</Tabs.List>

	<!-- At a glance: verdict and the three questions -->
	<Tabs.Content value="glance" class="flex flex-col gap-12">
		<section aria-label="Overview" class="flex flex-col gap-6">
			{#if isAi}
				<MonitorSummary
					dayOfConversation={sample.SAMPLE_DAY_OF_CONVERSATION}
					conversationLengthDays={sample.SAMPLE_CONVERSATION_LENGTH_DAYS}
					joined={sample.SAMPLE_JOINED}
					finished={sample.SAMPLE_FINISHED}
					biggestDropOffStep={worstStep.name}
					notifyHref="{base}/notifications"
					stepHref="{base}/design"
				/>
			{:else if mode === 'cool'}
				<div class="flex flex-col gap-3">
					<div
						role="group"
						aria-label="Preview timeline state"
						class="bg-card inline-flex self-start rounded-lg border p-1"
					>
						{#each [{ value: 'running', label: `Day ${sample.SAMPLE_DAY_OF_CONVERSATION}` }, { value: 'dayOne', label: 'Day 1 (no one yet)' }] as option (option.value)}
							<button
								type="button"
								aria-pressed={timelinePreview === option.value}
								onclick={() =>
									(timelinePreview = option.value as 'running' | 'dayOne')}
								class={cn(
									'min-h-9 rounded-md px-3 text-sm',
									timelinePreview === option.value
										? 'bg-sidebar text-sidebar-foreground'
										: 'text-muted-foreground'
								)}
							>
								{option.label}
							</button>
						{/each}
					</div>
					<MonitorTimeline
						state={timelinePreview}
						day={sample.SAMPLE_DAY_OF_CONVERSATION}
						totalDays={sample.SAMPLE_CONVERSATION_LENGTH_DAYS}
						launchDate={sample.SAMPLE_LAUNCH_DATE}
						closingDate={sample.SAMPLE_CLOSING_DATE}
						events={sample.SAMPLE_RECRUITMENT_EVENTS}
						activeNow={sample.SAMPLE_ACTIVE_NOW}
						recruitHref="{base}/invites"
						stats={[
							{
								label: 'Recruited',
								value: String(sample.SAMPLE_RECRUITED),
								detail: '+12 this week'
							},
							{
								label: 'Joined',
								value: String(sample.SAMPLE_JOINED),
								detail: `${Math.round((sample.SAMPLE_JOINED / sample.SAMPLE_RECRUITED) * 100)}% of recruited`
							},
							{
								label: 'Finished',
								value: String(sample.SAMPLE_FINISHED),
								detail: `${Math.round((sample.SAMPLE_FINISHED / sample.SAMPLE_JOINED) * 100)}% of joined`
							},
							{
								label: 'Time to finish',
								value: `${sample.SAMPLE_MEDIAN_MINUTES_TO_FINISH} min`,
								detail: 'Typical, first to last step'
							}
						]}
					/>
				</div>
			{:else}
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
							name: 'Finished',
							amount: sample.SAMPLE_FINISHED,
							unit: null,
							meaning: `Completed every step. ${Math.round((sample.SAMPLE_FINISHED / sample.SAMPLE_JOINED) * 100)}% of everyone who joined`,
							explanation:
								'Counts people who reached the end of the last step. Someone who skipped an optional step still counts as finished.'
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
							name: 'Time to finish',
							amount: sample.SAMPLE_MEDIAN_MINUTES_TO_FINISH,
							unit: 'min',
							meaning: 'Typical time from first step to last',
							explanation:
								'The median: half of the people who finished took less time than this, half took longer. Time away from the page is not counted.'
						}
					]}
				/>
				<FirstVisitHint tourHref="/admin/info/tools" />
			{/if}
			<div class="grid grid-cols-1 gap-5 lg:grid-cols-3">
				<QuestionCard
					eyebrow="Working"
					question="Are people finishing?"
					value="{Math.round((sample.SAMPLE_FINISHED / sample.SAMPLE_JOINED) * 100)}%"
					unit="of joiners finished"
					status={isAi ? 'healthy' : undefined}
					statusLabel={isAi ? 'Healthy' : undefined}
					trendLabel="Finish rate per step"
					bars={sample.SAMPLE_STEPS.map((step) => ({
						label: step.shortName,
						fullLabel: step.name,
						percent:
							step.reached === 0
								? 0
								: Math.round((step.finished / step.reached) * 100),
						flagged: step.name === worstStep.name
					}))}
					aiInsight={isAi ? sample.SAMPLE_AI_TEXT.working : undefined}
					actionLabel="Review {worstStep.name}"
					actionHref="{base}/design"
					onShowDetails={() => showTab('journey')}
					detailsTab="Journey"
				/>
				<QuestionCard
					eyebrow="Recruitment"
					question="Are enough people joining?"
					value={String(sample.SAMPLE_JOINED)}
					unit="of {sample.SAMPLE_TARGET} target"
					status={isAi ? 'attention' : undefined}
					statusLabel={isAi ? 'Behind pace' : undefined}
					trendLabel="Running total of signups against the target"
					aiInsight={isAi ? sample.SAMPLE_AI_TEXT.recruitment : undefined}
					actionLabel="Open Recruit"
					actionHref="{base}/invites"
					onShowDetails={() => showTab('recruitment')}
					detailsTab="Recruitment"
				>
					{#snippet visual()}
						<TargetSparkline
							signups={sample.SAMPLE_SIGNUPS}
							target={sample.SAMPLE_TARGET}
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
					status={isAi
						? underRepresentedCount > 0
							? 'attention'
							: 'healthy'
						: undefined}
					statusLabel={isAi
						? underRepresentedCount > 0
							? 'Gaps'
							: 'Well covered'
						: undefined}
					trendLabel="Share of participants compared with the local population"
					aiInsight={isAi ? sample.SAMPLE_AI_TEXT.reach : undefined}
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
		</section>
	</Tabs.Content>

	<!-- Pulse: what's happening right now, and who to follow up with -->
	<Tabs.Content value="pulse" class="flex flex-col gap-2">
		<MonitorStatsBar
			variant="secondary"
			stats={[
				{
					name: 'Active now',
					amount: sample.SAMPLE_ACTIVE_NOW,
					unit: null,
					live: true,
					details: [
						`${sample.SAMPLE_RECRUITED} people recruited`,
						'2 in Thinking space, 1 in Survey'
					]
				},
				{ name: 'Joined today', amount: 10, unit: null },
				{ name: 'Finished today', amount: 3, unit: null }
			]}
		/><TodayJourneys steps={sample.SAMPLE_STEPS} journeys={sample.SAMPLE_TODAY_JOURNEYS} />
		<ActivityHeatmap
			days={sample.HEATMAP_DAYS}
			slots={sample.HEATMAP_SLOTS}
			counts={sample.SAMPLE_HEATMAP}
			showAdvice={isAi}
		/>

		<div class="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]">
			<LivePulse activity={sample.SAMPLE_ACTIVITY} />
			<FollowUpActions
				actions={[
					{
						count: worstStep.reached - worstStep.finished,
						who: `stuck at ${worstStep.name}`,
						why: "Started but haven't been back for 3+ days.",
						label: 'Send a reminder',
						href: `${base}/notifications`,
						kind: 'message',
						primary: true
					},
					{
						count: sample.SAMPLE_FINISHED,
						who: 'finished',
						why: 'Thank them, and let them know when the report is out.',
						label: 'Send a thank-you',
						href: `${base}/notifications`,
						kind: 'message'
					},
					{
						count: sample.SAMPLE_OPTED_IN_CONTACT,
						who: 'agreed to be contacted',
						why: 'Contact details for people who opted in.',
						label: 'Download contacts',
						href: `/api/conversation/${data.conversation.id}/contacts/export`,
						kind: 'download',
						disabled: !canExport
					},
					{
						count: sample.SAMPLE_DEMOGRAPHICS_SHARED,
						who: 'shared demographics',
						why: 'Profile data for participants in this conversation.',
						label: 'Download demographics',
						href: `/api/conversation/${data.conversation.id}/demographics/export`,
						kind: 'download',
						disabled: !canExport
					}
				]}
			/>
		</div>
	</Tabs.Content>

	<!-- Journey: where people get stuck -->
	<Tabs.Content value="journey" class="flex flex-col gap-6">
		<div class="flex flex-wrap items-end justify-between gap-4">
			<div class="flex flex-col gap-1">
				<h2 class="text-2xl font-semibold">Journey</h2>
				<p class="text-muted-foreground text-base">
					How people move through your steps, and where they stop.
				</p>
			</div>
			<div
				role="group"
				aria-label="Show counts as"
				class="bg-card inline-flex rounded-lg border p-1"
			>
				{#each [{ value: 'people', label: 'People' }, { value: 'percent', label: '% of joiners' }] as option (option.value)}
					<button
						type="button"
						aria-pressed={showFunnelAs === option.value}
						onclick={() => (showFunnelAs = option.value as 'people' | 'percent')}
						class={cn(
							'min-h-9 rounded-md px-3 text-sm',
							showFunnelAs === option.value
								? 'bg-sidebar text-sidebar-foreground'
								: 'text-muted-foreground'
						)}
					>
						{option.label}
					</button>
				{/each}
			</div>
		</div>
		{#if isAi}
			<DropOffCallout
				step={worstStep}
				aiExplanation={sample.SAMPLE_AI_TEXT.dropOff}
				{typicalMinutes}
				notifyHref="{base}/notifications"
				stepHref="{base}/design"
			/>
		{/if}
		<StepFunnel
			steps={sample.SAMPLE_STEPS}
			joined={sample.SAMPLE_JOINED}
			worstStep={worstStep.name}
			showAs={showFunnelAs}
		/>
	</Tabs.Content>

	<!-- Recruitment: who is joining, from where, and whether they reflect the population -->
	<Tabs.Content value="recruitment" class="flex flex-col gap-12">
		<section aria-labelledby="recruitment-heading" class="flex flex-col gap-6">
			<div class="flex flex-col gap-1">
				<h2 id="recruitment-heading" class="text-2xl font-semibold">Recruitment</h2>
				<p class="text-muted-foreground text-base">
					Are enough people joining, and which recruitment efforts brought them in?
				</p>
			</div>
			<RecruitmentTarget
				showForecast={isAi}
				joined={sample.SAMPLE_JOINED}
				target={sample.SAMPLE_TARGET}
				{evenPaceToday}
				closingDate={sample.SAMPLE_CLOSING_DATE}
				daysLeft={sample.SAMPLE_CONVERSATION_LENGTH_DAYS -
					sample.SAMPLE_DAY_OF_CONVERSATION}
				projectedAtClose={sample.SAMPLE_PROJECTED_AT_CLOSE}
				editTargetHref="{base}/configure"
			/>
			{#if isAi}
				<AiInsight class="bg-card rounded-xl border p-5">
					<p class="text-base leading-relaxed">{sample.SAMPLE_AI_TEXT.signups}</p>
				</AiInsight>
			{/if}
			<SignupsOverTimeChart
				signups={sample.SAMPLE_SIGNUPS}
				targetPace={sample.SAMPLE_TARGET_PACE}
				projection={sample.SAMPLE_PROJECTION}
				events={sample.SAMPLE_RECRUITMENT_EVENTS}
				target={sample.SAMPLE_TARGET}
			/>
			<RecruitmentChannels channels={sample.SAMPLE_CHANNELS} manageHref="{base}/invites" />
		</section>

		<section aria-labelledby="reach-heading" class="flex flex-col gap-6">
			<div class="flex flex-col gap-1">
				<h2 id="reach-heading" class="text-2xl font-semibold">Reach</h2>
				<p class="text-muted-foreground text-base">
					Who you're hearing from, compared with the population you want to hear from.
				</p>
			</div>
			<RepresentationBars
				dimensions={sample.SAMPLE_REPRESENTATION}
				participantCount={sample.SAMPLE_JOINED}
			/>
		</section>
	</Tabs.Content>
</Tabs.Root>
