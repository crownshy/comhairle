<script lang="ts">
	import PolisInsights from '$lib/reports/polis/PolisInsights.svelte';
	import ThinkingSpaceInsights from '$lib/reports/thinking-space/ThinkingSpaceInsights.svelte';
	import PrioritizationInsights from '$lib/reports/prioritization/PrioritizationInsights.svelte';
	import SurveyInsights from '$lib/reports/survey/SurveyInsights.svelte';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { Button } from '$lib/components/ui/button';
	import { ChevronRight, MonitorPlay } from '@lucide/svelte';

	let { data } = $props();

	let step = $derived(data.step);

	// The Room display is a public, chrome-free page meant for a projector, so it
	// opens in its own window rather than inside the admin shell.
	const roomDisplayUrl = $derived(
		`/conversations/${page.params.conversation_id}/room-display/${step.id}`
	);
</script>

<svelte:head>
	<title>Insights for {step.name} - Comhairle Admin</title>
</svelte:head>

<nav class="text-muted-foreground mb-4 flex items-center gap-2 text-base">
	<a
		href={resolve('/(admin)/admin/conversations/[conversation_id]/insights', {
			conversation_id: data.conversation.id
		})}
		class="hover:text-foreground"
	>
		Insights
	</a>
	<ChevronRight class="size-4" />
	<span class="text-foreground">{step.name}</span>
</nav>

<!-- Thinking space -->
{#if data.thinkingSpace}
	<ThinkingSpaceInsights {...data.thinkingSpace} />
{/if}

<!-- Polis -->
{#if data.polis}
	<PolisInsights
		conversationId={data.conversation.id}
		workflowStepId={step.id}
		reportData={data.polis.reportData ?? null}
		statementAux={data.polis.statementAux ?? []}
		summary={data.polis.summary ?? null}
	>
		{#snippet actions()}
			<Button
				class="w-full"
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
{#if data.prioritization}
	<PrioritizationInsights {step} {...data.prioritization} />
{/if}

<!-- Survey -->
{#if data.survey}
	<SurveyInsights
		data={data.survey}
		conversationId={data.conversation.id}
		workflowStepId={step.id}
	/>
{/if}
