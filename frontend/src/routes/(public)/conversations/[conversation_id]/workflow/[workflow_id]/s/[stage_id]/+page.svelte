<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import ProcessDates from '$lib/components/ProcessDates.svelte';
	import FeedbackModal from '$lib/components/FeedbackModal.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import StepSelector from '$lib/components/StepSelector.svelte';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { report_url, workflow_step_url } from '$lib/urls';
	import { apiClient } from '$lib/api/client';
	import { addDays, parseISO } from 'date-fns';

	let { data }: PageProps = $props();
	let { conversation, step, workflow_steps, user } = data;
	let startDate = $derived(parseISO(conversation.created_at));
	let endDate = $derived(addDays(startDate, 30));

	async function stepComplete() {
		try {
			await apiClient.SetUserProgress('done', {
				params: {
					workflow_id: step.workflow_id,
					conversation_id: conversation.id,
					workflow_step_id: step.id
				},
				headers: { 'Content-Type': 'application/json' }
			});

			let next_step = workflow_steps.find((ws) => ws.step_order === step!.step_order + 1);
			if (next_step) {
				let url = workflow_step_url(conversation.id, step.workflow_id, next_step.step_order);
				//For some reason goto isnt working here. Need to figure out why
				window.location.href = url;
			} else {
				//For some reason goto isnt working here
				window.location.href = report_url(conversation.id, step.workflow_id);
			}
		} catch (e) {
			if (e instanceof Error) {
				console.warn(e.message);
			}
			notifications.send({
				message: 'Something unexpected happend. Try again shorlty',
				priority: 'ERROR'
			});
		}
	}
</script>

<div class="flex h-full flex-col pt-10">
	{#if conversation && step}
		<div class="hidden md:block">
			<Breadcrumbs {conversation} workflow_step={step} />
		</div>

		<div class="flex w-full grow flex-col md:grid md:grid-cols-[1fr_300px] md:gap-x-10">
			<h1
				class="text-4xl font-bold md:col-start-1 md:col-end-2 md:row-start-1 md:row-end-1 md:text-6xl"
			>
				{step.name}
			</h1>
			<div class="flex grow flex-col md:row-start-2">
				<p class="mb-4">
					{step.description}
				</p>
				<div class="grow">
					{#if step.tool_config.type === Learn.TOOL_NAME}
						<Learn.UserUI onDone={stepComplete} pages={step.tool_config.pages} user_id={user.id} />
					{/if}
					{#if step.tool_config.type === Polis.TOOL_NAME}
						<Polis.UserUI
							user_id={user.id}
							polis_id={step.tool_config.poll_id}
							polis_url={step.tool_config.server_url}
							onDone={stepComplete}
						/>
					{/if}
					{#if step.tool_config.type === HeyForm.TOOL_NAME}
						<HeyForm.UserUI
							userId={user.id}
							surveyId={step.tool_config.survey_id}
							surveyURL={step.tool_config.survey_url}
							onDone={stepComplete}
						/>
					{/if}
				</div>
			</div>
			<div class="hidden w-full md:row-start-2 md:block md:flex md:flex-col md:gap-10">
				<ProcessDates {startDate} {endDate} />
				<div class="b-green-950 mt-2 border-b-2 border-t-4 p-4 text-xl font-bold">
					Part of {conversation.title}
					<StepSelector steps={workflow_steps} currentStep={step} />
				</div>
				<FeedbackModal conversation_id={conversation.id} />
			</div>
		</div>
	{:else}
		<h1>Failed to find conversation</h1>
	{/if}
</div>
