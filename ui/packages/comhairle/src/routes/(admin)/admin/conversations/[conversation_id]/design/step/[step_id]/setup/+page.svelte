<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import ThinkingSpaceManage from '$lib/tools/thinking_space/ThinkingSpaceManage.svelte';
	import ElicitationBotManage from '$lib/tools/elicitation_bot/ElicitationBotManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import * as Prioritization from '$lib/tools/prioritization';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import ParticipantViewSplit from '$lib/components/admin/ParticipantViewSplit.svelte';
	import ParticipantScreen from '$lib/components/admin/ParticipantScreen.svelte';
	import StepShell from '$lib/components/participant/StepShell.svelte';
	import StepToolBody from '$lib/components/participant/StepToolBody.svelte';
	import StepPager from '$lib/components/participant/StepPager.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { splitSlides } from '$lib/step-brief/splitSlides';
	import { segmentFill } from '$lib/step-brief/segmentFill';
	import { learningAssistantAvailable } from '$lib/components/LearningAssistant/availability';
	import * as m from '$lib/paraglide/messages';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);
	let workflow = $derived(data.workflows[0]);
	let toolConfig = $derived(data.toolConfig);

	/**
	 * HeyForm's participant UI is a third-party iframe, so there is no component to render
	 * inert and nothing a participant view could honestly show (ADR-0030).
	 */
	let previewable = $derived(
		toolConfig?.type !== undefined && toolConfig.type !== HeyForm.TOOL_NAME
	);

	let availableDocuments = $derived(data.availableDocuments ?? []);
	let hasKnowledgeBaseDocs = $derived(availableDocuments.length > 0);
	let assistantAvailable = $derived(
		learningAssistantAvailable(conversation, hasKnowledgeBaseDocs)
	);

	let sortedSteps = $derived(
		[...(data.workflowSteps ?? [])].sort((a, b) => a.stepOrder - b.stepOrder)
	);
	let viewedIndex = $derived(sortedSteps.findIndex((s) => s.id === step?.id));

	let chromeSteps = $derived<StepItem[]>([
		{
			id: 'landing',
			name: m.landing_before_you_start(),
			status: 'completed',
			isIntro: true
		},
		...sortedSteps.map((s, index): StepItem => {
			let status: StepItem['status'] = 'upcoming';
			if (index === viewedIndex) status = 'current';
			else if (index < viewedIndex) status = 'completed';
			return { id: s.id, name: s.name, status };
		})
	]);

	let stepLabel = $derived(
		`${m.step_position_label({ current: viewedIndex + 1, total: sortedSteps.length })}: ${step?.name ?? ''}`
	);

	// The brief only matters here for its chip in the chrome: a step with no description has
	// no brief to reopen, so no chip.
	let hasBrief = $derived(splitSlides(step?.description).length > 0);

	/**
	 * The step as a participant meets it, before they have done anything. Learn and Lived
	 * Experience let you leave straight away; every other tool holds the pager until it says
	 * otherwise, and an optional step offers Skip instead.
	 */
	let canProceed = $derived(
		toolConfig?.type === Learn.TOOL_NAME || toolConfig?.type === LivedExperience.TOOL_NAME
	);
	let forwardMode = $derived<'next' | 'skip'>(canProceed || step?.required ? 'next' : 'skip');
</script>

{#snippet manage()}
	{#if step && toolConfig?.type === 'learn'}
		<LearnManage
			conversationId={conversation.id}
			{conversation}
			isLive={conversation.isLive}
			workflowStep={step}
		/>
	{/if}

	{#if step && toolConfig?.type === 'polis'}
		<PolisManage
			{toolConfig}
			conversationId={conversation.id}
			workflowId={workflow.id}
			workflowStepId={step.id}
			isLive={conversation.isLive}
		/>
	{/if}

	{#if step && toolConfig?.type === 'heyform'}
		<HeyFormManage
			conversation_id={conversation.id}
			workflow_id={step.workflowId}
			workflow_step_id={step.id}
			survey_url={toolConfig.server_url}
			survey_id={toolConfig.survey_id}
			admin_user={toolConfig.admin_user}
			admin_password={toolConfig.admin_password}
			workspace_id={toolConfig.workspace_id}
			project_id={toolConfig.project_id}
		/>
	{/if}

	{#if toolConfig?.type === 'stories'}
		<LivedExperienceManage />
	{/if}

	{#if step && toolConfig?.type === 'thinkingspace'}
		<ThinkingSpaceManage
			{conversation}
			workflowId={step.workflowId}
			workflowStep={step}
			isLive={conversation.isLive}
		/>
	{/if}

	{#if step && toolConfig?.type === 'elicitationbot'}
		<ElicitationBotManage
			conversationId={conversation.id}
			workflowId={step.workflowId}
			workflowStep={step}
			isLive={conversation.isLive}
		/>
	{/if}

	{#if step && toolConfig?.type === Prioritization.TOOL_NAME}
		{#key step.id}
			<Prioritization.ManageUI
				{conversation}
				workflowId={step.workflowId}
				workflowStep={step}
			/>
		{/key}
	{/if}
{/snippet}

{#if previewable && step}
	<ParticipantViewSplit
		description="The step as a participant meets it, before they have done anything."
	>
		{#snippet editor()}
			{@render manage()}
		{/snippet}

		{#snippet screens({ device, scale })}
			<ParticipantScreen {device} {scale}>
				<StepShell
					class="h-full"
					chrome={{
						steps: chromeSteps,
						currentIndex: viewedIndex + 1,
						label: stepLabel,
						fill: segmentFill({ phase: 'body', slideIndex: 0, slideCount: 1 }),
						assistantAvailable,
						showSupport: false,
						onBrief: hasBrief ? () => {} : undefined
					}}
				>
					{#snippet content()}
						<StepToolBody
							{toolConfig}
							{conversation}
							workflowStep={step}
							userId={data.user.id}
							{availableDocuments}
							{hasKnowledgeBaseDocs}
						/>
					{/snippet}

					{#snippet bar()}
						<StepPager
							{forwardMode}
							canGoBack={true}
							canGoForward={canProceed || !step.required}
							onBack={() => {}}
							onForward={() => {}}
						/>
					{/snippet}
				</StepShell>
			</ParticipantScreen>
		{/snippet}
	</ParticipantViewSplit>
{:else}
	{@render manage()}
{/if}
