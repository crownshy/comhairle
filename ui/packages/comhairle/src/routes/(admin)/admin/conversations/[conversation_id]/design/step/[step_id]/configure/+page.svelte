<script lang="ts">
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import StepPreviewDialog from '$lib/components/admin/StepPreviewDialog.svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);

	/** The editor's unsaved description, mirrored into the preview frame as it is typed. */
	let draftDescription = $state('');
</script>

{#if step}
	<div class="flex max-w-3xl flex-col gap-8">
		<StepPreviewDialog
			conversationId={conversation.id}
			workflowId={step.workflowId}
			stepId={step.id}
			description={draftDescription}
		/>

		<CommonStepConfig
			conversation_id={conversation.id}
			{conversation}
			{step}
			inline
			onDraftDescriptionChange={(value) => (draftDescription = value)}
		/>
	</div>
{/if}
