<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import ThinkingSpaceManage from '$lib/tools/thinking_space/ThinkingSpaceManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import { useAdminLayoutSlots } from '../../../useAdminLayoutSlots.svelte.js';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Button } from '$lib/components/ui/button';
	import { Pencil } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	let { data } = $props();

	let editMetadataOpen = $state(false);

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

	useAdminLayoutSlots({
		title: titleSnippet,
		breadcrumbs: breadcrumbSnippet
	});
	let pageTitle = $derived(`Edit Step: ${step?.name ?? 'Step'}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

{#snippet titleSnippet()}
	<div class="flex min-w-0 flex-1 flex-col gap-2">
		<div class="flex flex-wrap items-center gap-3">
			<h1 class="text-3xl font-semibold sm:text-4xl">
				{getTextInLocale(
					step?.translations?.name,
					conversation.primaryLocale ?? 'en',
					step?.name ?? ''
				) || 'Unnamed step'}
			</h1>
			<Button
				onclick={() => (editMetadataOpen = true)}
				class="bg-sidebar text-sidebar-foreground hover:bg-sidebar/90 h-8 rounded-full px-3 text-xs"
			>
				<Pencil class="size-3.5" />
				Edit
			</Button>
		</div>
		{#if step?.description || step?.translations?.description}
			<ContentRenderer
				content={getTextInLocale(
					step?.translations?.description,
					conversation.primaryLocale ?? 'en',
					step?.description ?? ''
				)}
				class="text-muted-foreground text-base"
			/>
		{/if}
	</div>
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
	<Breadcrumb.Item>{step?.name}</Breadcrumb.Item>
{/snippet}

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

{#if step && toolConfig?.type === 'elicitationbot'}
	<ThinkingSpaceManage
		conversationId={conversation.id}
		workflowId={step.workflowId}
		workflowStep={step}
		isLive={conversation.isLive}
	/>
{/if}
