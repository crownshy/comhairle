<script lang="ts">
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import ProcessDates from '$lib/components/ProcessDates.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import StepSelector from '$lib/components/StepSelector.svelte';

	let { data } = $props();
	let { conversation, step, workflow_steps, user } = data;
	console.log({ conversation, step, workflow_steps });

	function stepComplete() {}
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
						polis_id={step.tool_config.polis_id}
						polis_url={step.tool_config.polis_url}
					/>
				{/if}
				{#if step.tool_config.type === HeyForm.TOOL_NAME}
					<HeyForm.UserUI
						userId={user.id}
						surveyId={step.tool_config.survey_id}
						surveyURL={step.tool_config.survey_url}
						onDone={() => {
							stepComplete;
						}}
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
