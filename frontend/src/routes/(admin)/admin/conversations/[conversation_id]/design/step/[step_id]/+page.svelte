<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import EliciationBotManage from '$lib/tools/elicitation_bot/ElicitationBotManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import type { WorkflowStep } from '$lib/api/api.js';
	import { useAdminLayoutSlots } from '../../../useAdminLayoutSlots.svelte.js';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflow_steps = $derived(data.workflow_steps);

	let step = $derived(workflow_steps.find((s: WorkflowStep) => s.id === step_id));
	let nextStep = $derived(
		workflow_steps.find((s: WorkflowStep) => s.step_order === step.step_order + 1)
	);
	let prevStep = $derived(
		workflow_steps.find((s: WorkflowStep) => s.step_order === step.step_order - 1)
	);
	let toolConfig = $derived(conversation.is_live ? step.tool_config : step.preview_tool_config);

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

{#snippet titleSnippet()}
	<h1 class="text-4xl font-bold">Design: {step.name}</h1>
	<AdminPrevNextControls
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
			: { name: 'Design', url: `/admin/conversations/${conversation.id}/design` }}
	/>
{/snippet}

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>
		<Breadcrumb.Link href={`/admin/conversations/${conversation.id}/design`}>
			Design
		</Breadcrumb.Link>
	</Breadcrumb.Item>
	<Breadcrumb.Separator />
	<Breadcrumb.Item>{step.name}</Breadcrumb.Item>
{/snippet}

<CommonStepConfig conversation_id={conversation.id} {step} />

{#if toolConfig.type === 'learn'}
	<LearnManage
		conversation_id={conversation.id}
		{conversation}
		isLive={conversation.is_live}
		workflow_step={step}
	/>
{/if}

{#if toolConfig.type === 'polis'}
	<PolisManage
		polis_id={toolConfig.poll_id}
		polis_url={toolConfig.server_url}
		admin_user={toolConfig.admin_user}
		admin_password={toolConfig.admin_password}
		workflow_step_id={step.id}
	/>
{/if}

{#if toolConfig.type === 'heyform'}
	<HeyFormManage
		conversation_id={conversation.id}
		workflow_id={step.workflow_id}
		workflow_step_id={step.id}
		survey_url={toolConfig.survey_url}
		survey_id={toolConfig.survey_id}
		admin_user={toolConfig.admin_user}
		admin_password={toolConfig.admin_password}
		workspace_id={toolConfig.workspace_id}
		project_id={toolConfig.project_id}
	/>
{/if}

{#if toolConfig.type === 'stories'}
	<LivedExperienceManage />
{/if}

{#if toolConfig.type === 'elicitationbot'}
	<EliciationBotManage
		conversationId={conversation.id}
		workflowId={step.workflow_id}
		workflowStep={step}
		isLive={conversation.is_live}
	/>
{/if}
