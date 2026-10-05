<script lang="ts">
	import PolisInsights from '$lib/reports/polis/PolisInsights.svelte';
	import ThinkingSpaceInsights from '$lib/reports/thinking-space/ThinkingSpaceInsights.svelte';
	import PrioritizationInsights from '$lib/reports/prioritization/PrioritizationInsights.svelte';
	import SurveyInsights from '$lib/reports/survey/SurveyInsights.svelte';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { MonitorPlay } from '@lucide/svelte';

	let { data } = $props();

	let step = $derived(data.step);

	// The Room display is a public, chrome-free page meant for a projector, so it
	// opens in its own window rather than inside the admin shell.
	const roomDisplayUrl = $derived(
		step ? `/conversations/${page.params.conversation_id}/room-display/${step.id}` : null
	);
</script>

<!-- Thinking space -->
{#if data.thinkingSpace}
	<ThinkingSpaceInsights {...data.thinkingSpace} />
{/if}

<!-- Polis -->
{#if data.polis && step && roomDisplayUrl}
	<PolisInsights
		workflowStepId={step.id}
		reportData={data.polis.reportData ?? null}
		statementAux={data.polis.statementAux ?? []}
	>
		{#snippet actions()}
			<Button
				href={roomDisplayUrl}
				target="_blank"
				rel="noopener"
				size="sm"
				variant="outline"
			>
				<MonitorPlay class="size-4" />
				Open room display
			</Button>
		{/snippet}
	</PolisInsights>
{/if}

<!-- Prioritization -->
{#if data.prioritization && step}
	<PrioritizationInsights {step} {...data.prioritization} />
{/if}

<!-- Survey -->
{#if data.survey && step}
	<SurveyInsights data={data.survey} />
{/if}
