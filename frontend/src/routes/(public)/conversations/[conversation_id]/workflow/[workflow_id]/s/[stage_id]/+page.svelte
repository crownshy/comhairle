<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import ProcessDates from '$lib/components/ProcessDates.svelte';
	import FeedbackModal from '$lib/components/FeedbackModal.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import StepSelector from '$lib/components/StepSelector.svelte';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { report_url, workflow_step_url } from '$lib/urls';
	import { apiClient } from '$lib/api/client';
	import { addDays, parseISO } from 'date-fns';
	import { video } from 'carta-plugin-video';
	import { Markdown, Carta } from 'carta-md';
	import DOMPurify from 'isomorphic-dompurify';

	import { ws } from '$lib/api/websockets.svelte';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';

	let { data }: PageProps = $props();
	let { user } = data;
	let step = $derived(data.step);
	let conversation = $derived(data.conversation);
	let workflow_steps = $derived(data.workflow_steps);
	let startDate = $derived(parseISO(conversation.created_at));
	let endDate = $derived(addDays(startDate, 30));

	let carta = new Carta({
		sanitizer: DOMPurify.sanitize,
		extensions: [video()]
	});

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
				goto(url);
				// window.location.href = url;
			} else {
				//For some reason goto isnt working here
				goto(report_url(conversation.id, step.workflow_id));
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
	{#if conversation && step}
		<div class="flex flex-row gap-2">
			{#each workflow_steps as workflow_step}
				<div
					class="bg-brand flex flex-col items-center gap-3 rounded-4xl px-10 py-3 text-sm text-[#ffffff]"
					class:bg-brand={workflow_step.id === step.id}
					class:bg-sky-500={workflow_step.id !== step.id}
				>
					<div class="border-secondary h-[24px] w-[24px] rounded-[100%] border-8 bg-white"></div>
					<p class="text-center">
						{workflow_step.name}
					</p>
				</div>
			{/each}
		</div>

		<div class="flex w-full grow flex-col gap-y-5 md:grid md:grid-cols-1 md:gap-x-10">
			<div class="mt-10 flex flex-col items-center gap-y-5">
				<h1 class="text-2xl">{conversation.title}</h1>
				<h2
					class="text-center text-4xl font-bold md:col-start-1 md:col-end-2 md:row-start-1 md:row-end-1 md:text-6xl"
				>
					{step.name}
				</h2>
				<div class="prose mx-auto">
					<Markdown {carta} value={step.description} />
				</div>
			</div>
			<div class=" flex grow flex-col md:row-start-2">
				{#if !step.required}
					<Button onclick={stepComplete} class="mx-auto" variant="secondary">Skip this step</Button>
				{/if}
				<div class="my-10 w-full grow">
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
					{#if step.tool_config.type === LivedExperience.TOOL_NAME}
						<LivedExperience.UserUI onDone={stepComplete} />
					{/if}
					{#if step.tool_config.type === ElicitationBot.TOOL_NAME}
						<ElicitationBot.UserUI onDone={stepComplete} />
					{/if}
				</div>
			</div>
		</div>
	{:else}
		<h1>Failed to find conversation</h1>
	{/if}
</div>
