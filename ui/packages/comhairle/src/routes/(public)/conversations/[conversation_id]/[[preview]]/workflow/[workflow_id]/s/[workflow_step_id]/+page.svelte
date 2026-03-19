<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { createCarta } from '$lib/utils/carta';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import StepSelector, { type StepItem } from '$lib/components/StepSelector.svelte';
	import StepHeader from '$lib/components/StepHeader.svelte';

	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { thank_you_page, next_workflow_step_url, workflow_step_url } from '$lib/urls';

	let { data }: PageProps = $props();
	let { user } = data;
	let workflow_id = $derived(data.workflow_id);
	let workflowStep = $derived(data.workflowStep);
	let conversation = $derived(data.conversation);
	let workflowSteps = $derived(data.workflowSteps);
	let userProgress = $derived(data.userProgress ?? []);

	let toolConfig = $derived(
		conversation.isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig
	);

	let carta = createCarta();
	let pageTitle = $derived(workflowStep?.name ?? 'Workflow Step');

	let stepItems = $derived<StepItem[]>(
		[...workflowSteps]
			.sort((a, b) => a.stepOrder - b.stepOrder)
			.map((ws) => {
				const progress = userProgress.find((p) => p.workflowStepId === ws.id);
				const isCurrent = ws.id === workflowStep.id;
				const isCompleted = progress?.status === 'done';
				const isBefore = ws.stepOrder < workflowStep.stepOrder;

				let status: StepItem['status'];
				if (isCurrent) {
					status = 'current';
				} else if (isCompleted) {
					// TODO: Determine navigability from API — for now, completed steps
					// before current are navigable, completed steps after are locked
					status = isBefore ? 'completed' : 'completed-locked';
				} else if (isBefore) {
					// Step is before current but not marked done — treat as completed
					// (they must have passed through it to reach current)
					status = 'completed';
				} else {
					status = 'upcoming';
				}

				const isPreview = !conversation.isLive;
				const href =
					status === 'completed'
						? workflow_step_url(conversation.id, workflow_id, ws.id, isPreview)
						: undefined;

				return { id: ws.id, name: ws.name, status, href };
			})
	);

	let currentStepNumber = $derived(stepItems.findIndex((s) => s.status === 'current') + 1);

	let prevStepHref = $derived(() => {
		const currentIdx = stepItems.findIndex((s) => s.status === 'current');
		if (currentIdx <= 0) return undefined;
		const sortedSteps = [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder);
		const prevWs = sortedSteps[currentIdx - 1];
		if (!prevWs) return undefined;
		const isPreview = !conversation.isLive;
		return workflow_step_url(conversation.id, workflow_id, prevWs.id, isPreview);
	});

	let currentNextAction = $state<(() => void) | undefined>(undefined);

	function handleNextAction(fn: () => void) {
		currentNextAction = fn;
	}

	function goToThankYouPage() {
		goto(thank_you_page(conversation.id, workflowStep.id));
	}

	async function stepComplete() {
		try {
			if (conversation.isLive) {
				await apiClient.SetUserProgress('done', {
					params: {
						workflow_id: workflowStep.workflowId,
						conversation_id: conversation.id,
						workflow_step_id: workflowStep.id
					},
					headers: { 'Content-Type': 'application/json' }
				});

				goto(next_workflow_step_url(conversation.id, workflowStep.workflowId));
			} else {
				let next = workflowSteps.find((w) => w.stepOrder === workflowStep.stepOrder + 1);
				if (next) {
					let next_step_url = workflow_step_url(
						conversation.id,
						workflow_id,
						next.id,
						!conversation.isLive
					);
					goto(next_step_url);
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
		}
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<div class="flex flex-col items-center sm:gap-6 sm:py-2 md:py-12">
	{#if conversation && workflowStep}
		<div
			class="mx-auto flex w-full items-center justify-center px-6 pt-5 pb-2 md:order-2 md:px-0 md:pt-0 md:pb-0"
		>
			<StepSelector steps={stepItems} />
		</div>

		<div class="w-full md:order-1 md:px-0">
			<StepHeader
				{currentStepNumber}
				totalSteps={stepItems.length}
				title={workflowStep.name}
				description={workflowStep.description}
				prevHref={prevStepHref()}
				onNext={currentNextAction ?? stepComplete}
			/>
		</div>

		<div class="flex w-full grow flex-col gap-y-2 md:order-3">
			<div class="flex grow flex-col">
				{#if !workflowStep.required}
					<Button onclick={stepComplete} class="mx-auto" variant="secondary"
						>Skip this step</Button
					>
				{/if}
				<div class="my-10 w-full grow">
					{#if toolConfig.type === Learn.TOOL_NAME}
						<Learn.UserUI
							onDone={stepComplete}
							pages={toolConfig.pages}
							user_id={user.id}
							onNextAction={handleNextAction}
						/>
					{/if}
					{#if toolConfig.type === Polis.TOOL_NAME}
						<Polis.UserUI
							user_id={user.id}
							polis_id={toolConfig.poll_id}
							polis_url={toolConfig.server_url}
							onDone={stepComplete}
						/>
					{/if}
					{#if toolConfig.type === HeyForm.TOOL_NAME}
						{#key workflowStep.id}
							<HeyForm.UserUI
								userId={user.id}
								surveyId={toolConfig.survey_id}
								surveyURL={toolConfig.survey_url}
								onDone={stepComplete}
							/>
						{/key}
					{/if}
					{#if toolConfig.type === LivedExperience.TOOL_NAME}
						<LivedExperience.UserUI onDone={stepComplete} />
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
