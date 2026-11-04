<script lang="ts">
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import { embeddedSectionBlocksForStep } from '$lib/reports/embeds';
	import { useTour } from '$lib/tours/useTour.svelte';
	import PublicInReport from './PublicInReport.svelte';
	import { stepConfigureTour } from './tour';

	useTour(() => stepConfigureTour);

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);
	let reportEmbeds = $derived(data.reportEmbeds);
</script>

{#if step}
	<CommonStepConfig conversation_id={conversation.id} {conversation} {step} inline>
		{#snippet extraFields()}
			{#if reportEmbeds}
				<PublicInReport
					conversationId={conversation.id}
					embedded={embeddedSectionBlocksForStep(
						reportEmbeds.sectionBlocks,
						step.id,
						data.toolConfig?.type
					)}
					reportIsPublic={reportEmbeds.isPublic}
				/>
			{/if}
		{/snippet}
	</CommonStepConfig>
{/if}
