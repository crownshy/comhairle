<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import ProcessDates from '$lib/components/ProcessDates.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import StepSelector from '$lib/components/StepSelector.svelte';
	import type { PageProps } from './$types';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';
	import { report_url, workflow_step_url } from '$lib/urls';

	let { data }: PageProps = $props();
	let { conversation, step, workflow_steps, user } = data;


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

{#if conversation && step}
	<Breadcrumbs {conversation} workflow_step={step} />

	<div class="grid w-full grid-cols-5 gap-8">
		<h1 class="col-start-1 col-end-4 row-start-1 row-end-1 text-8xl font-bold">{step.name}</h1>
		<h2 class="col-start-1 col-end-4 row-start-2 row-end-2 text-xl font-bold">
			{step.name}
		</h2>
		<div class="col-start-1 col-end-4 row-start-3">
			<p>
				{step.description}
			</p>
			<div class="h-[600px]">
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
		<div class="col-start-4 col-end-6 row-start-3 w-full">
			<ProcessDates startDate={new Date(2025, 1, 1)} endDate={new Date(2025, 1, 28)} />
			<div class="b-green-950 mt-2 border-b-2 border-t-4 p-4 text-xl font-bold">
				Part of {conversation.title}
			</div>
			<div>
				<StepSelector steps={workflow_steps} currentStep={step} />
			</div>
		</div>
	</div>
{:else}
	<h1>Failed to find conversation</h1>
{/if}
