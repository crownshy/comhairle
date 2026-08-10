<script lang="ts">
	import PolisInsights from '$lib/reports/polis/PolisInsights.svelte';
	import ThinkingSpaceInsights from '$lib/reports/thinking-space/ThinkingSpaceInsights.svelte';
	import PrioritizationInsights from '$lib/reports/prioritization/PrioritizationInsights.svelte';
	import SurveyInsights from '$lib/reports/survey/SurveyInsights.svelte';

	let { data } = $props();

	let step = $derived(data.step);
</script>

<!-- Thinking space -->
{#if data.thinkingSpace}
	<ThinkingSpaceInsights {...data.thinkingSpace} />
{/if}

<!-- Polis -->
{#if data.polis && step}
	<PolisInsights
		workflowStepId={step.id}
		reportData={data.polis.reportData ?? null}
		statementAux={data.polis.statementAux ?? []}
	/>
{/if}

<!-- Prioritization -->
{#if data.prioritization && step}
	<PrioritizationInsights {step} {...data.prioritization} />
{/if}

<!-- Survey -->
{#if data.survey && step}
	<SurveyInsights data={data.survey} />
{/if}
