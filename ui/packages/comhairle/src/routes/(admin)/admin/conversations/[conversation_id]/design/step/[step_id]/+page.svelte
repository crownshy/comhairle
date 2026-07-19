<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import PolisInsights from '$lib/tools/polis/PolisInsights.svelte';
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import ThinkingSpaceManage from '$lib/tools/thinking_space/ThinkingSpaceManage.svelte';
	import ElicitationBotManage from '$lib/tools/elicitation_bot/ElicitationBotManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import * as Prioritization from '$lib/tools/prioritization';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import { page } from '$app/state';

	let { data } = $props();

	let availableDocuments = $state<ComhairleDocument[]>([]);

	$effect(() => {
		const cid = data.conversation?.id;
		if (!cid) return;
		apiClient
			.ListDocuments({ params: { conversation_id: cid } })
			.then((docs) => {
				availableDocuments = docs.filter((d) => d.parse_status === 'DONE');
			})
			.catch(() => {
				availableDocuments = [];
			});
	});

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflow = $derived(data.workflows[0]);
	let workflowSteps = $derived(data.workflowSteps);

	let step = $derived(workflowSteps.find((s) => s.id === step_id));
	let toolConfig = $derived(
		step ? (conversation.isLive ? step.toolConfig : step.previewToolConfig) : null
	);

	let pageTitle = $derived(`Edit Step: ${step?.name ?? 'Step'}`);

	let isPolis = $derived(toolConfig?.type === 'polis');
	// The sub-tab strip (Row 4) is now rendered from `+layout.svelte`; this page only reads
	// which sub-tab is active to switch content. See +layout.ts for the strip's data.
	let subtab = $derived(page.url.searchParams.get('subtab') ?? 'configure');
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

{#if step && subtab === 'configure'}
	<CommonStepConfig conversation_id={conversation.id} {conversation} {step} inline />
{/if}

{#if step && subtab === 'setup' && toolConfig?.type === 'learn'}
	<LearnManage
		conversationId={conversation.id}
		{conversation}
		isLive={conversation.isLive}
		workflowStep={step}
	/>
{/if}

{#if subtab === 'setup' && toolConfig?.type === 'polis'}
	<PolisManage
		{toolConfig}
		conversationId={conversation.id}
		workflowId={workflow.id}
		workflowStepId={step.id}
		isLive={conversation.isLive}
	/>
{:else if step && isPolis && subtab === 'moderation'}
	<PolisModeration workflowStepId={step.id} statements={data.statementAux ?? []} />
{:else if step && isPolis && subtab === 'insights'}
	<PolisInsights
		workflowStepId={step.id}
		reportData={data.reportData ?? null}
		statementAux={data.statementAux ?? []}
	/>
{/if}

{#if subtab === 'setup' && toolConfig?.type === 'heyform'}
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

{#if subtab === 'setup' && toolConfig?.type === 'stories'}
	<LivedExperienceManage />
{/if}

{#if step && subtab === 'setup' && toolConfig?.type === 'thinkingspace'}
	<ThinkingSpaceManage
		conversationId={conversation.id}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && subtab === 'setup' && toolConfig?.type === 'elicitationbot'}
	<ElicitationBotManage
		conversationId={conversation.id}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && subtab === 'setup' && toolConfig?.type === Prioritization.TOOL_NAME}
	{#key step.id}
		<Prioritization.ManageUI
			conversationId={conversation.id}
			workflowId={step.workflowId}
			workflowStep={step}
			conversation={{
				primaryLocale: conversation.primaryLocale,
				isLive: conversation.isLive,
				supportedLanguages: conversation.supportedLanguages
			}}
		/>
	{/key}
{/if}
