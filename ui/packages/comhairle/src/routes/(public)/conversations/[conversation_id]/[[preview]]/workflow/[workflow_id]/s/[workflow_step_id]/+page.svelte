<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ThinkingSpace from '$lib/tools/thinking_space/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import * as Prioritization from '$lib/tools/prioritization/index.js';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import StepSelector, { type StepItem } from '$lib/components/StepSelector.svelte';
	import StepHeader from '$lib/components/StepHeader.svelte';
	import StepHeaderSkeleton from '$lib/components/StepHeaderSkeleton.svelte';

	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { goto } from '$app/navigation';
	import { thank_you_page, next_workflow_step_url, workflow_step_url } from '$lib/urls';
	import { page, navigating } from '$app/state';
	import LearnArticleSkeleton from '$lib/tools/learn/LearnArticleSkeleton.svelte';
	import LearnTutorSkeleton from '$lib/tools/learn/LearnTutorSkeleton.svelte';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	const url = $derived(page.url);
	const queryString = $derived(url.search);

	let { data }: PageProps = $props();
	let { user, preview: isPreview } = data;
	let workflow_id = $derived(data.workflow_id);
	let workflowStep = $derived(data.workflowStep);
	let conversation = $derived(data.conversation);
	let workflowSteps = $derived(data.workflowSteps);

	let toolConfig = $derived(
		conversation.isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig
	);

	let pageTitle = $derived(workflowStep?.name ?? 'Workflow Step');

	let sortedSteps = $derived([...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder));

	let actualCurrentStep = $derived(
		conversation.isLive
			? (sortedSteps.find((ws) => ws.progressStatus !== 'done') ?? null)
			: workflowStep
	);

	let isRevisiting = $derived(workflowStep.progressStatus === 'done');

	let availableDocuments = $state<ComhairleDocument[]>([]);
	let loadedDocumentsConversationId = $state<string | null>(null);

	$effect(() => {
		const conversationId = conversation?.id ?? null;
		if (!conversationId) {
			availableDocuments = [];
			loadedDocumentsConversationId = null;
			return;
		}

		if (loadedDocumentsConversationId === conversationId) return;

		loadedDocumentsConversationId = conversationId;
		apiClient
			.ListDocuments({ params: { conversation_id: conversationId } })
			.then((docs) => {
				if (loadedDocumentsConversationId !== conversationId) return;
				availableDocuments = docs.filter((d) => d.parse_status === 'DONE');
			})
			.catch(() => {
				if (loadedDocumentsConversationId !== conversationId) return;
				availableDocuments = [];
			});
	});

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

	let currentStepNumber = $derived(sortedSteps.findIndex((ws) => ws.id === workflowStep.id) + 1);

	let prevStepHref = $derived.by(() => {
		const viewedIdx = sortedSteps.findIndex((ws) => ws.id === workflowStep.id);
		if (viewedIdx <= 0) return undefined;
		const prevItem = stepItems[viewedIdx - 1];
		if (!prevItem || prevItem.status !== 'completed') return undefined;
		return prevItem.href;
	});

	let currentNextAction = $state<(() => void) | undefined>(undefined);
	let currentPrevAction = $state<(() => void) | undefined>(undefined);
	let canProceed = $state(false);
	let isSubmitting = $state(false);

	$effect(() => {
		workflowStep.id;
		isSubmitting = false;
	});

	$effect(() => {
		const type = toolConfig.type;
		if (type === Learn.TOOL_NAME || type === LivedExperience.TOOL_NAME) {
			canProceed = true;
		} else {
			canProceed = false;
		}
		if (type !== Learn.TOOL_NAME) {
			currentNextAction = undefined;
			currentPrevAction = undefined;
		}
	});

	function handleNextAction(fn: () => void) {
		currentNextAction = fn;
	}

	function handlePrevAction(fn: (() => void) | undefined) {
		currentPrevAction = fn;
	}

	function handleCanContinueChange(value: boolean) {
		canProceed = value;
	}

	function goToThankYouPage() {
		goto(thank_you_page(conversation.id, workflow_id, !conversation.isLive) + queryString);
	}

	async function stepComplete() {
		if (isSubmitting) return;
		isSubmitting = true;

		if (isRevisiting) {
			const isPreview = !conversation.isLive;
			const currentIdx = sortedSteps.findIndex((ws) => ws.id === workflowStep.id);
			const nextRevisitable = sortedSteps.slice(currentIdx + 1).find((ws) => ws.canRevisit);
			const target = nextRevisitable ?? actualCurrentStep;
			if (target) {
				goto(
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

				goto(
					next_workflow_step_url(conversation.id, workflowStep.workflowId) + queryString
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
					goto(next_step_url + queryString);
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
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<div class="flex flex-col items-center sm:py-2 md:py-10">
	{#if conversation && workflowStep}
		<div
			class="mx-auto flex w-full items-center justify-center px-6 pt-5 pb-2 md:px-0 md:pt-0 md:pb-0"
		>
			<StepSelector steps={stepItems} />
		</div>

		<div class="mt-2 w-full md:mt-6 md:px-0">
			{#if navigating.to}
				<StepHeaderSkeleton />
			{:else}
				<StepHeader
					{currentStepNumber}
					totalSteps={stepItems.length}
					title={workflowStep.name}
					description={workflowStep.description}
					prevHref={prevStepHref}
					onPrev={currentPrevAction}
					onNext={currentNextAction ?? stepComplete}
					nextDisabled={!canProceed}
					nextLoading={isSubmitting}
					boldDescription={toolConfig.type === Polis.TOOL_NAME}
					{availableDocuments}
					conversationId={conversation.id}
				/>
			{/if}
		</div>

		<div class="flex w-full grow flex-col gap-y-2 md:order-3">
			<div class="flex grow flex-col">
				{#if !workflowStep.required}
					<Button
						onclick={stepComplete}
						disabled={isSubmitting}
						class="mx-auto"
						variant="secondary"
					>
						{#if isSubmitting}
							<Spinner class="mr-2 size-4" />
						{/if}
						Skip this step
					</Button>
				{/if}
				<div class="mb-10 w-full grow">
					{#if navigating.to}
						<LearnArticleSkeleton />
						{#if conversation?.chatBotId && conversation.enableQaChatBot}
							<div class="mx-auto mt-6 w-full max-w-[65ch]">
								<LearnTutorSkeleton />
							</div>
						{/if}
					{:else if toolConfig.type === Learn.TOOL_NAME}
						{#key workflowStep.id}
							<Learn.UserUI
								onDone={stepComplete}
								pages={toolConfig.pages}
								user_id={user.id}
								onNextAction={handleNextAction}
								onPrevAction={handlePrevAction}
								{conversation}
								{isSubmitting}
							/>
						{/key}
					{/if}
					{#if toolConfig?.type === Polis.TOOL_NAME}
						<Polis.UserUI
							user_id={user.id}
							polis_id={toolConfig.poll_id}
							polis_url={toolConfig.server_url}
							requiredVotes={toolConfig.required_votes}
							workflowStepId={workflowStep.id}
							onDone={stepComplete}
							onCanContinueChange={handleCanContinueChange}
							showRemainingStatementCount={toolConfig.show_remaining_statements}
						/>
					{/if}
					{#if toolConfig.type === HeyForm.TOOL_NAME}
						{#key workflowStep.id}
							<HeyForm.UserUI
								userId={user.id}
								surveyId={toolConfig.survey_id}
								surveyURL={toolConfig.survey_url}
								serverURL={toolConfig.server_url}
								onDone={stepComplete}
							/>
						{/key}
					{/if}
					{#if toolConfig.type === LivedExperience.TOOL_NAME}
						<LivedExperience.UserUI onDone={stepComplete} />
					{/if}
					{#if toolConfig.type === ThinkingSpace.TOOL_NAME}
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
					{/if}
					{#if toolConfig.type === ElicitationBot.TOOL_NAME}
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
					{/if}
					{#if toolConfig.type === Prioritization.TOOL_NAME}
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
							/>
						{/key}
					{/if}
				</div>
			</div>
		</div>
	{:else}
		<h1>Failed to find conversation</h1>
	{/if}
</div>
