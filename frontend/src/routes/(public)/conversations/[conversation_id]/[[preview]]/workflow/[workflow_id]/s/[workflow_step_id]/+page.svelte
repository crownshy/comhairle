<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '$lib/api/client';
	import { Markdown } from 'carta-md';
	import { createCarta } from '$lib/utils/carta';

	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { thank_you_page, next_workflow_step_url, workflow_step_url } from '$lib/urls';

	let { data }: PageProps = $props();
	let { user } = data;
	let workflow_id = $derived(data.workflow_id);
	let workflow_step = $derived(data.workflow_step);
	let conversation = $derived(data.conversation);
	let workflow_steps = $derived(data.workflow_steps);

	let tool_config = $derived(
		conversation.isLive ? workflow_step.tool_config : workflow_step.preview_tool_config
	);


	let carta = createCarta();

	function goToThankYouPage() {
		goto(thank_you_page(conversation.id, workflow_step.id));
	}

	async function stepComplete() {
		try {
			if (conversation.isLive) {
				await apiClient.SetUserProgress('done', {
					params: {
						workflow_id: workflow_step.workflow_id,
						conversation_id: conversation.id,
						workflow_step_id: workflow_step.id
					},
					headers: { 'Content-Type': 'application/json' }
				});

				goto(next_workflow_step_url(conversation.id, workflow_step.workflow_id));
			} else {
				let next = workflow_steps.find(
					(w) => w.step_order === workflow_step.step_order + 1
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
	{#if conversation && workflow_step}
		<div class="hidden flex-row gap-2 md:flex">
			{#each workflow_steps as step (step.id)}
				<div
					class="bg-brand flex flex-col items-center gap-3 rounded-4xl px-10 py-3 text-sm text-[#ffffff]"
					class:bg-brand={workflow_step.id === step.id}
					class:bg-sky-500={workflow_step.id !== step.id}
				>
					<div
						class="border-secondary h-[24px] w-[24px] rounded-[100%] border-8 bg-white"
					></div>
					<p class="text-center">
						{step.name}
					</p>
				</div>
			{/each}
		</div>

		<div class="flex w-full grow flex-col gap-y-2 md:grid md:grid-cols-1 md:gap-x-10">
			<div class="mt-10 flex flex-col items-center gap-y-2">
				<h2
					class="text-center text-4xl font-bold md:col-start-1 md:col-end-2 md:row-start-1 md:row-end-1 md:text-3xl"
				>
					{workflow_step.name}
				</h2>
				<div class="prose-sm prose-p:text-base prose-li:text-base mx-auto">
					{#key workflow_step.description}
						<Markdown {carta} value={workflow_step.description} />
					{/key}
				</div>
			</div>
			<div class="flex grow flex-col md:row-start-2">
				{#if !workflow_step.required}
					<Button onclick={stepComplete} class="mx-auto" variant="secondary"
						>Skip this step</Button
					>
				{/if}
				<div class="my-10 w-full grow">
					{#if tool_config.type === Learn.TOOL_NAME}
						<Learn.UserUI
							onDone={stepComplete}
							pages={tool_config.pages}
							user_id={user.id}
						/>
					{/if}
					{#if tool_config.type === Polis.TOOL_NAME}
						<Polis.UserUI
							user_id={user.id}
							polis_id={tool_config.poll_id}
							polis_url={tool_config.server_url}
							onDone={goToThankYouPage}
						/>
					{/if}
					{#if tool_config.type === HeyForm.TOOL_NAME}
						{#key workflow_step.id}
							<HeyForm.UserUI
								userId={user.id}
								surveyId={tool_config.survey_id}
								surveyURL={tool_config.survey_url}
								onDone={stepComplete}
							/>
						{/key}
					{/if}
					{#if tool_config.type === LivedExperience.TOOL_NAME}
						<LivedExperience.UserUI onDone={stepComplete} />
					{/if}
					{#if tool_config.type === ElicitationBot.TOOL_NAME}
						{#key workflow_step.id}
							<ElicitationBot.UserUI
								conversationId={conversation.id}
								workflowId={workflow_step.workflow_id}
								workflowStepId={workflow_step.id}
								userId={user.id}
								topic={tool_config.topic}
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
