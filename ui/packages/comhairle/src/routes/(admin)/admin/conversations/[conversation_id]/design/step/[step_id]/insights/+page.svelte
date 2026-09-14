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
	<div class="flex flex-wrap items-center gap-2 pb-4">
		<Button href={roomDisplayUrl} target="_blank" rel="noopener" variant="outline">
			<MonitorPlay />
			Open room display
		</Button>
		<Button href="{roomDisplayUrl}?mode=demo" target="_blank" rel="noopener" variant="ghost">
			Demo run
		</Button>
	</div>
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
