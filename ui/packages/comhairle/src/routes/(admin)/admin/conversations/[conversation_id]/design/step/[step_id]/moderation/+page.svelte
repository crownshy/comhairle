<script lang="ts">
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import { rejectReasonsForStep } from '$lib/moderation/moderationPolicy';

	let { data } = $props();

	let step = $derived(data.step);
	let rejectReasons = $derived(
		rejectReasonsForStep(data.toolConfig, data.moderationPolicies, data.defaultRejectReasons)
	);
</script>

{#if step}
	<PolisModeration
		workflowStepId={step.id}
		statements={data.statementAux ?? []}
		{rejectReasons}
		defaultRejectReasons={data.defaultRejectReasons}
	/>
{/if}
