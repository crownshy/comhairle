<script lang="ts">
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import { stepPreviewDraft } from '$lib/step-brief/previewDraft.svelte';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);
</script>

{#if step}
	<div class="max-w-3xl">
		<CommonStepConfig
			conversation_id={conversation.id}
			{conversation}
			{step}
			inline
			onDraftDescriptionChange={(value) => stepPreviewDraft.set(step.id, value)}
		/>
	</div>
{/if}
