<script lang="ts">
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import { rejectReasonsForStep } from '$lib/moderation/moderationPolicy';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { stepModerationTour } from './tour';

	useTour(() => stepModerationTour);

	let { data } = $props();

	let step = $derived(data.step);
	let rejectReasons = $derived(
		rejectReasonsForStep(data.toolConfig, data.moderationPolicies, data.defaultRejectReasons)
	);
</script>

{#if step}
	<PolisModeration
		conversationId={data.conversation.id}
		workflowStepId={step.id}
		statements={data.statementAux ?? []}
		{rejectReasons}
		defaultRejectReasons={data.defaultRejectReasons}
	/>
{/if}
