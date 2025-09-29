<script lang="ts">
	import LearnManage from '$lib/tools/learn/LearnManage.svelte';
	import PolisManage from '$lib/tools/polis/PolisManage.svelte';
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '$lib/api/client';
	import TextArea from '$lib/components/ui/textarea/textarea.svelte';
	import { notifications } from '$lib/notifications.svelte.js';
	import { invalidateAll } from '$app/navigation';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import LivedExperienceManage from '$lib/tools/lived_experince/LivedExperinceManage.svelte';
	import type { WorkflowStep } from '$lib/api/api.js';
	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step_id = $derived(data.step_id);
	let workflow_steps = $derived(data.workflow_steps);
	let step = $derived(workflow_steps.find((s: WorkflowStep) => s.id === step_id));
</script>

<CommonStepConfig conversation_id={conversation.id} {step} />

{#if step.tool_config.type === 'learn'}
	<LearnManage
		conversation_id={conversation.id}
		pages={step.tool_config.pages}
		workflow_step={step}
	/>
{/if}

{#if step.tool_config.type === 'polis'}
	<PolisManage
		polis_id={step.tool_config.poll_id}
		polis_url={step.tool_config.server_url}
		admin_user={step.tool_config.admin_user}
		admin_password={step.tool_config.admin_password}
		workflow_step_id={step.id}
	/>
{/if}

{#if step.tool_config.type === 'heyform'}
	<HeyFormManage
		conversation_id={conversation.id}
		workflow_id={step.workflow_id}
		workflow_step_id={step.id}
		survey_url={step.tool_config.survey_url}
		survey_id={step.tool_config.survey_id}
		admin_user={step.tool_config.admin_user}
		admin_password={step.tool_config.admin_password}
		workspace_id={step.tool_config.workspace_id}
		project_id={step.tool_config.project_id}
	/>
{/if}

{#if step.tool_config.type === 'stories'}
	<LivedExperienceManage />
{/if}
