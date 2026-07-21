<script lang="ts">
	import PolisInsights from '$lib/tools/polis/PolisInsights.svelte';

	let { data } = $props();

	let step = $derived(data.step);
</script>

{#if step}
	<PolisInsights
		workflowStepId={step.id}
		reportData={data.reportData ?? null}
		statementAux={data.statementAux ?? []}
	/>
{/if}
