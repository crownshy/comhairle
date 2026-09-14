<script lang="ts">
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import { moderationPolicyFromMetadata } from '$lib/moderation/moderationPolicy';

	let { data } = $props();

	let step = $derived(data.step);
	let rejectReasons = $derived(
		moderationPolicyFromMetadata(data.conversation.metadata).rejectReasons
	);
</script>

{#if step}
	<PolisModeration
		workflowStepId={step.id}
		statements={data.statementAux ?? []}
		{rejectReasons}
	/>
{/if}
