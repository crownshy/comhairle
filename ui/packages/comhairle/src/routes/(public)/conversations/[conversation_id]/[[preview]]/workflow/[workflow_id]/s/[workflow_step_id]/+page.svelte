<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ThinkingSpace from '$lib/tools/thinking_space/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import * as Prioritization from '$lib/tools/prioritization/index.js';
	import type { ComponentProps } from 'svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import * as m from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import StepShell from './StepShell.svelte';
	import StepProgressBar from './StepProgressBar.svelte';
	import StepPager from './StepPager.svelte';
	import StepHeader from './StepHeader.svelte';
	import StepHeaderSkeleton from './StepHeaderSkeleton.svelte';
	import type { StepItem } from './stepItems';
	import { STEP_COLUMN_CLASS } from './styles';

	import { goto } from '$app/navigation';
	import { thank_you_page, next_workflow_step_url, workflow_step_url } from '$lib/urls';
	import { page, navigating } from '$app/state';
	import LearnArticleSkeleton from '$lib/tools/learn/LearnArticleSkeleton.svelte';
	import { delayedFlag } from '$lib/utils/delayedFlag.svelte';
	import LearningAssistantSkeleton from '$lib/components/LearningAssistant/LearningAssistantSkeleton.svelte';

	const url = $derived(page.url);
	const queryString = $derived(url.search);

	let { data } = $props();
	let {
		user,
		preview: isPreview,
		workflow_id,
		workflowStep,
		workflowSteps,
		conversation,
		availableDocuments,
		hasKnowledgeBaseDocs
	} = $derived(data);

	let toolConfig = $derived(
		conversation.isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig
	);

	let pageTitle = $derived(workflowStep?.name ?? 'Workflow Step');

	let sortedSteps = $derived(workflowSteps.toSorted((a, b) => a.stepOrder - b.stepOrder));

	let actualCurrentStep = $derived(
		conversation.isLive
			? (sortedSteps.find((ws) => ws.progressStatus !== 'done') ?? null)
			: workflowStep
	);

	let isRevisiting = $derived(workflowStep.progressStatus === 'done');

	let stepItems = $derived<StepItem[]>(
		sortedSteps.map((ws) => {
			const isCurrent = actualCurrentStep ? ws.id === actualCurrentStep.id : false;
			const isCompleted = ws.progressStatus === 'done';
			const actualCurrentOrder = actualCurrentStep?.stepOrder ?? Infinity;
			const isBefore = ws.stepOrder < actualCurrentOrder;
			const canRevisit = ws.canRevisit;

			const passedThrough = isCompleted || isBefore;

			let status: StepItem['status'];
			if (isCurrent) {
				status = 'current';
			} else if (passedThrough && canRevisit) {
				status = 'completed';
			} else if (passedThrough) {
				status = 'completed-locked';
			} else {
				status = 'upcoming';
			}

			const href =
				status === 'completed'
					? workflow_step_url(conversation.id, workflow_id, ws.id, isPreview) +
						queryString
					: undefined;

			return { id: ws.id, name: ws.name, status, href };
		})
	);

	let viewedIndex = $derived(sortedSteps.findIndex((ws) => ws.id === workflowStep.id));
	let currentStepNumber = $derived(viewedIndex + 1);

	// Empty until the step is done. Filling within a step is ADR-0047 parts 2 and 3.
	let fill = $derived(isRevisiting ? 1 : 0);

	// Mid-navigation `data` still describes the step we're leaving, so the skeleton is picked
	// from the destination's tool.
	let navigatingToToolType = $derived.by(() => {
		const targetId = navigating.to?.params?.workflow_step_id;
		if (!targetId) return undefined;
		const target = sortedSteps.find((ws) => ws.id === targetId);
		if (!target) return undefined;
		return conversation.isLive ? target.toolConfig?.type : target.previewToolConfig?.type;
	});

	let showNavigationSkeleton = delayedFlag(() => navigating.to !== null, 150);

	let prevStepHref = $derived.by(() => {
		if (viewedIndex <= 0) return undefined;
		const prevItem = stepItems[viewedIndex - 1];
		if (!prevItem || prevItem.status !== 'completed') return undefined;
		return prevItem.href;
	});

	// Tools report back through callbacks. Each report is tagged with its step so a report from
	// the step we just left can never leak into the next one.
	type StepScoped<T> = { stepId: string; value: T };

	let toolNextAction = $state.raw<StepScoped<() => void>>();
	let toolPrevAction = $state.raw<StepScoped<(() => void) | undefined>>();
	let toolCanContinue = $state.raw<StepScoped<boolean>>();
	let submittingStepId = $state<string>();

	function forThisStep<T>(scoped: StepScoped<T> | undefined): T | undefined {
		return scoped?.stepId === workflowStep.id ? scoped.value : undefined;
	}

	let currentNextAction = $derived(forThisStep(toolNextAction));
	let currentPrevAction = $derived(forThisStep(toolPrevAction));
	let toolNeedsNoSignal = $derived.by(() => {
		const type = toolConfig?.type;
		return type === Learn.TOOL_NAME || type === LivedExperience.TOOL_NAME;
	});
	let canProceed = $derived(forThisStep(toolCanContinue) ?? toolNeedsNoSignal);
	let isSubmitting = $derived(submittingStepId === workflowStep.id);

	function handleNextAction(fn: () => void) {
		toolNextAction = { stepId: workflowStep.id, value: fn };
	}

	function handlePrevAction(fn: (() => void) | undefined) {
		toolPrevAction = { stepId: workflowStep.id, value: fn };
	}

	function handleCanContinueChange(value: boolean) {
		toolCanContinue = { stepId: workflowStep.id, value };
	}

	// Learn's own pages come before the step boundary (ADR-0047).
	let stepCanAdvance = $derived(currentNextAction !== undefined || canProceed || isRevisiting);
	let canGoBack = $derived(currentPrevAction !== undefined || prevStepHref !== undefined);
	let canGoForward = $derived(stepCanAdvance || !workflowStep.required);
	let forwardMode = $derived<ComponentProps<typeof StepPager>['forwardMode']>(
		stepCanAdvance || workflowStep.required ? 'next' : 'skip'
	);

	// Every destination keeps the current query string (such as ?embed=true), and the lint rule
	// only accepts a bare resolve() call, so `resolve(...) + queryString` would still fail it.
	function navigateTo(href: string, options?: Parameters<typeof goto>[1]) {
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		return goto(href, options);
	}

	function goBack() {
		if (currentPrevAction) {
			currentPrevAction();
			return;
		}
		if (prevStepHref) navigateTo(prevStepHref);
	}

	function goForward() {
		if (currentNextAction) {
			currentNextAction();
			return;
		}
		stepComplete();
	}

	function goToThankYouPage() {
		navigateTo(
			thank_you_page(conversation.id, workflow_id, !conversation.isLive) + queryString
		);
	}

	async function stepComplete() {
		if (isSubmitting) return;
		submittingStepId = workflowStep.id;

		if (isRevisiting) {
			const isPreview = !conversation.isLive;
			const currentIdx = sortedSteps.findIndex((ws) => ws.id === workflowStep.id);
			const nextRevisitable = sortedSteps.slice(currentIdx + 1).find((ws) => ws.canRevisit);
			const target = nextRevisitable ?? actualCurrentStep;
			if (target) {
				navigateTo(
					workflow_step_url(conversation.id, workflow_id, target.id, isPreview) +
						queryString
				);
			} else {
				goToThankYouPage();
			}
			return;
		}

		try {
			if (conversation.isLive) {
				await apiClient.SetUserProgress(
					{ status: 'done' },
					{
						params: {
							workflow_id: workflowStep.workflowId,
							conversation_id: conversation.id,
							workflow_step_id: workflowStep.id
						},
						headers: { 'Content-Type': 'application/json' }
					}
				);

				// Not invalidate(): this page's load would redirect a finished step to /next,
				// rejecting the invalidate and firing a spurious error toast.
				await navigateTo(
					next_workflow_step_url(conversation.id, workflowStep.workflowId) + queryString,
					{ invalidateAll: true }
				);
			} else {
				let next = workflowSteps.find((w) => w.stepOrder === workflowStep.stepOrder + 1);
				if (next) {
					let next_step_url = workflow_step_url(
						conversation.id,
						workflow_id,
						next.id,
						!conversation.isLive
					);
					navigateTo(next_step_url + queryString);
				} else {
					goToThankYouPage();
				}
			}
		} catch (e) {
			if (e instanceof Error) {
				console.warn(e.message);
			}
			notifications.send({
				message: 'Something unexpected happened. Try again shortly',
				priority: 'ERROR'
			});
			submittingStepId = undefined;
		}
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

{#if conversation && workflowStep && user}
	<StepShell class="min-h-0 grow">
		{#snippet header()}
			<header class="bg-background pt-3 md:pt-4">
				<div class={STEP_COLUMN_CLASS}>
					<StepProgressBar steps={stepItems} currentIndex={viewedIndex} {fill} />
				</div>
				{#if showNavigationSkeleton.current}
					<StepHeaderSkeleton />
				{:else}
					<StepHeader
						{currentStepNumber}
						totalSteps={stepItems.length}
						title={workflowStep.name}
						description={workflowStep.description}
						{availableDocuments}
						conversationId={conversation.id}
					/>
				{/if}
			</header>
		{/snippet}

		{#snippet content()}
			<div class={cn(STEP_COLUMN_CLASS, 'flex min-h-full flex-col pt-2')}>
				{#if showNavigationSkeleton.current}
					{#if navigatingToToolType === HeyForm.TOOL_NAME}
						<HeyForm.UserUISkeleton />
					{:else}
						<LearnArticleSkeleton />
						{#if conversation?.chatBotId && conversation.enableQaChatBot && hasKnowledgeBaseDocs}
							<div class="mx-auto mt-6 w-full max-w-[65ch]">
								<LearningAssistantSkeleton />
							</div>
						{/if}
					{/if}
				{:else if toolConfig?.type === Learn.TOOL_NAME}
					{#key workflowStep.id}
						<Learn.UserUI
							onDone={stepComplete}
							pages={toolConfig.pages}
							user_id={user.id}
							onNextAction={handleNextAction}
							onPrevAction={handlePrevAction}
							{conversation}
							{availableDocuments}
							{hasKnowledgeBaseDocs}
						/>
					{/key}
				{:else if toolConfig?.type === Polis.TOOL_NAME}
					{#key workflowStep.id}
						<Polis.UserUI
							user_id={user.id}
							polis_id={toolConfig.poll_id}
							polis_url={toolConfig.server_url}
							requiredVotes={toolConfig.required_votes}
							workflowStepId={workflowStep.id}
							{isPreview}
							onCanContinueChange={handleCanContinueChange}
							showRemainingStatementCount={toolConfig.show_remaining_statements}
						/>
					{/key}
				{:else if toolConfig?.type === HeyForm.TOOL_NAME}
					{#key workflowStep.id}
						<HeyForm.UserUI
							userId={user.id}
							surveyId={toolConfig.survey_id}
							surveyURL={toolConfig.survey_url}
							serverURL={toolConfig.server_url}
							onDone={stepComplete}
						/>
					{/key}
				{:else if toolConfig?.type === LivedExperience.TOOL_NAME}
					<LivedExperience.UserUI onDone={stepComplete} />
				{:else if toolConfig?.type === ThinkingSpace.TOOL_NAME}
					{#key workflowStep.id}
						<ThinkingSpace.UserUI
							workflowStepId={workflowStep.id}
							workflowId={workflowStep.workflowId}
							conversationId={conversation.id}
							userId={user.id}
							topic={toolConfig.topic}
							rootQuestions={toolConfig.root_questions}
							followUpRoundsCount={toolConfig.follow_up_rounds_count}
							requestUserSharePermission={workflowStep.requestUserSharePermission}
							initialPermissionToShareWithOrganizers={data.permissionToShareWithOrganizers}
							progressStatus={workflowStep.progressStatus}
							onDone={stepComplete}
							onCanContinueChange={handleCanContinueChange}
						/>
					{/key}
				{:else if toolConfig?.type === ElicitationBot.TOOL_NAME}
					{#key workflowStep.id}
						<ElicitationBot.UserUI
							conversationId={conversation.id}
							workflowId={workflowStep.workflowId}
							workflowStepId={workflowStep.id}
							userId={user.id}
							topic={toolConfig.topic}
							onDone={stepComplete}
							onCanContinueChange={handleCanContinueChange}
						/>
					{/key}
				{:else if toolConfig?.type === Prioritization.TOOL_NAME}
					{#key workflowStep.id}
						<Prioritization.UserUI
							{workflowStep}
							conversation={{
								primaryLocale: conversation.primaryLocale,
								isLive: conversation.isLive,
								supportedLanguages: conversation.supportedLanguages
							}}
							participantId={user.id}
							onDone={stepComplete}
							onCanContinueChange={handleCanContinueChange}
						/>
					{/key}
				{/if}
			</div>
		{/snippet}

		{#snippet bar()}
			<StepPager
				{forwardMode}
				{canGoBack}
				{canGoForward}
				loading={isSubmitting}
				onBack={goBack}
				onForward={goForward}
			/>
		{/snippet}
	</StepShell>
{:else}
	<h1>{m.step_not_found()}</h1>
{/if}
