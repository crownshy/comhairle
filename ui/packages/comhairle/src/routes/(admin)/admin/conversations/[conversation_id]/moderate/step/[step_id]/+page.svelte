<script lang="ts">
	import { resolve } from '$app/paths';
	import { ChevronRight } from 'lucide-svelte';
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import { rejectReasonsForStep } from '$lib/moderation/moderationPolicy';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { stepModerationTour } from './tour';

	useTour(() => stepModerationTour);

	let { data } = $props();

	let rejectReasons = $derived(
		rejectReasonsForStep(data.toolConfig, data.moderationPolicies, data.defaultRejectReasons)
	);
</script>

<svelte:head>
	<title>Moderate {data.step.name} - Comhairle Admin</title>
</svelte:head>

<nav class="text-muted-foreground mb-4 flex items-center gap-2 text-base">
	<a
		href={resolve('/(admin)/admin/conversations/[conversation_id]/moderate', {
			conversation_id: data.conversation.id
		})}
		class="hover:text-foreground"
	>
		Moderate
	</a>
	<ChevronRight class="size-4" />
	<span class="text-foreground">{data.step.name}</span>
</nav>

{#if data.statementsFailed}
	<p class="text-destructive mb-4">Statements could not be loaded. Try Sync from Polis.</p>
{/if}

<PolisModeration
	conversationId={data.conversation.id}
	workflowStepId={data.step.id}
	statements={data.statements}
	{rejectReasons}
	defaultRejectReasons={data.defaultRejectReasons}
/>
