<script lang="ts">
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import StepPreviewPanel from '$lib/components/admin/StepPreviewPanel.svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);

	/** The editor's unsaved description, mirrored into the preview frame as it is typed. */
	let draftDescription = $state('');
</script>

{#if step}
	<!-- Side by side above lg: authoring a slide break is invisible in the editor, so the
		phone next to it is what makes it legible (ADR-0017). -->
	<div class="flex flex-col gap-10 lg:flex-row lg:items-start lg:gap-8">
		<div class="min-w-0 flex-1">
			<CommonStepConfig
				conversation_id={conversation.id}
				{conversation}
				{step}
				inline
				onDraftDescriptionChange={(value) => (draftDescription = value)}
			/>
		</div>
		<div class="w-full shrink-0 lg:w-[458px]">
			<div class="lg:sticky lg:top-6">
				<StepPreviewPanel
					conversationId={conversation.id}
					workflowId={step.workflowId}
					stepId={step.id}
					description={draftDescription}
				/>
			</div>
		</div>
	</div>
{/if}
