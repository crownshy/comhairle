<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import ThinkingSpaceManage from '$lib/tools/thinking_space/ThinkingSpaceManage.svelte';
	import ElicitationBotManage from '$lib/tools/elicitation_bot/ElicitationBotManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import * as Prioritization from '$lib/tools/prioritization';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Pencil } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	let { data } = $props();

	let editMetadataOpen = $state(false);
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
	let nextStep = $derived(
		step ? workflowSteps.find((s) => s.stepOrder === step.stepOrder + 1) : undefined
	);
	let prevStep = $derived(
		step ? workflowSteps.find((s) => s.stepOrder === step.stepOrder - 1) : undefined
	);
	let toolConfig = $derived(
		step ? (conversation.isLive ? step.toolConfig : step.previewToolConfig) : null
	);

	let pageTitle = $derived(`Edit Step: ${step?.name ?? 'Step'}`);
	let stepName = $derived(
		getTextInLocale(
			step?.translations?.name,
			conversation.primaryLocale ?? 'en',
			step?.name ?? ''
		) || 'Unnamed step'
	);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<div class="mb-6 flex w-full min-w-0 flex-col">
	<div class="flex w-full min-w-0 items-center gap-3">
		<h1
			class="min-w-0 flex-1 truncate text-3xl font-semibold sm:max-w-[40ch] sm:text-4xl"
			title={stepName}
		>
			{stepName}
		</h1>
		<Button
			onclick={() => (editMetadataOpen = true)}
			class="bg-sidebar text-sidebar-foreground hover:bg-sidebar/90 h-8 shrink-0 rounded-full px-3 text-xs"
		>
			<Pencil class="size-3.5" />
			Edit
		</Button>
	</div>
	{#if step?.description || step?.translations?.description}
		<div class="mt-2">
			<ContentRenderer
				content={getTextInLocale(
					step?.translations?.description,
					conversation.primaryLocale ?? 'en',
					step?.description ?? ''
				)}
				class="text-muted-foreground text-base"
				{availableDocuments}
				conversationId={conversation.id}
			/>
		</div>
	{/if}
	<div class="border-base-border mt-5 flex w-full border-t pt-4">
		<AdminPrevNextControls
			hidePrevLabel
			next={nextStep
				? {
						name: nextStep.name,
						url: `/admin/conversations/${conversation.id}/design/step/${nextStep.id}`
					}
				: {
						name: 'Setup Knowledge base',
						url: `/admin/conversations/${conversation.id}/knowledge-base`
					}}
			prev={prevStep
				? {
						name: prevStep.name,
						url: `/admin/conversations/${conversation.id}/design/step/${prevStep.id}`
					}
				: { name: 'Workflow', url: `/admin/conversations/${conversation.id}/design` }}
		/>
	</div>
</div>

{#if step}
	<CommonStepConfig
		conversation_id={conversation.id}
		{conversation}
		{step}
		headerless
		bind:open={editMetadataOpen}
	/>
{/if}

{#if step && toolConfig?.type === 'learn'}
	<LearnManage
		conversationId={conversation.id}
		{conversation}
		isLive={conversation.isLive}
		workflowStep={step}
	/>
{/if}

{#if toolConfig?.type === 'polis'}
	<PolisManage
		{toolConfig}
		conversationId={conversation.id}
		workflowId={workflow.id}
		workflowStepId={step.id}
		isLive={conversation.isLive}
	/>
{/if}

{#if toolConfig?.type === 'heyform'}
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

{#if toolConfig?.type === 'stories'}
	<LivedExperienceManage />
{/if}

{#if step && toolConfig?.type === 'thinkingspace'}
	<ThinkingSpaceManage
		conversationId={conversation.id}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && toolConfig?.type === 'elicitationbot'}
	<ElicitationBotManage
		conversationId={conversation.id}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}

{#if step && toolConfig?.type === Prioritization.TOOL_NAME}
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
