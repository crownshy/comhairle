<script lang="ts">
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';

	let { data } = $props();

	let step = $derived(data.step);
</script>

{#if step}
	<PolisModeration workflowStepId={step.id} statements={data.statementAux ?? []} />
{/if}
