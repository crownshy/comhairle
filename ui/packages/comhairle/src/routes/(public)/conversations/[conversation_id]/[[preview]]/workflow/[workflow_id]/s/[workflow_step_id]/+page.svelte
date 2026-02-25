<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crown-shy/api-client/client';
	import { createCarta } from '$lib/utils/carta';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import StepSelector from '$lib/components/StepSelector.svelte';

	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { thank_you_page, next_workflow_step_url, workflow_step_url } from '$lib/urls';

	let { data }: PageProps = $props();
	let { user } = data;
	let workflow_id = $derived(data.workflow_id);
	let workflowStep = $derived(data.workflowStep);
	let conversation = $derived(data.conversation);
	let workflowSteps = $derived(data.workflowSteps);

	let toolConfig = $derived(
		conversation.isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig
	);


	let carta = createCarta();

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
				let next = workflowSteps.find(
					(w) => w.stepOrder === workflowStep.stepOrder + 1
				);
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

<div class="flex flex-col items-center pt-10">
	{#if conversation && workflowStep}
		<StepSelector steps={workflowSteps} currentStepId={workflowStep.id} />

		<div class="flex w-full grow flex-col gap-y-2 md:grid md:grid-cols-1 md:gap-x-10">
			<div class="mt-10 flex flex-col items-center gap-y-2">
				<h2
					class="text-center text-4xl font-bold md:col-start-1 md:col-end-2 md:row-start-1 md:row-end-1 md:text-3xl"
				>
					{workflowStep.name}
				</h2>
				<div class="prose-sm prose-p:text-base prose-li:text-base mx-auto">
					{#key workflowStep.description}
						<ContentRenderer content={workflowStep.description} />
					{/key}
				</div>
			</div>
			<div class="flex grow flex-col md:row-start-2">
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
						/>
					{/if}
					{#if toolConfig.type === Polis.TOOL_NAME}
						<Polis.UserUI
							user_id={user.id}
							polis_id={toolConfig.poll_id}
							polis_url={toolConfig.server_url}
							onDone={goToThankYouPage}
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
