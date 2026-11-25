<script lang="ts">
	import { resolve } from '$app/paths';
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import { rejectReasonsForStep } from '$lib/moderation/moderationPolicy';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { stepModerationTour } from './tour';

	let { data } = $props();

	const canModerate = $derived(
		permissions.can('conversation', 'conversation_moderate', data.conversation.id)
	);

	// Moderators get the rules dialog as their first-visit welcome, so their tour waits for a replay.
	useTour(
		() => stepModerationTour(canModerate),
		() => ({ autoplay: !canModerate })
	);

	let rejectReasons = $derived(
		rejectReasonsForStep(data.toolConfig, data.moderationPolicies, data.defaultRejectReasons)
	);
</script>

<svelte:head>
	<title>Moderate {data.step.name} - Comhairle Admin</title>
</svelte:head>

{#if data.hasOtherSteps}
	<nav aria-label="Breadcrumb" class="mb-2">
		<a
			href={resolve('/(admin)/admin/conversations/[conversation_id]/moderate', {
				conversation_id: data.conversation.id
			})}
			class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 text-base"
		>
			<ChevronLeft class="size-4" />
			All steps
		</a>
	</nav>
{/if}

{#if data.statementsFailed}
	<p class="text-destructive mb-4">Statements could not be loaded. Try reloading the page.</p>
{/if}

<PolisModeration
	conversationId={data.conversation.id}
	conversationTitle={data.conversation.title}
	stepName={data.step.name}
	workflowStepId={data.step.id}
	statements={data.statements}
	translations={data.translations}
	{rejectReasons}
	defaultRejectReasons={data.defaultRejectReasons}
	step={{
		conversationId: data.conversation.id,
		workflowId: data.step.workflowId,
		workflowStepId: data.step.id,
		isLive: data.conversation.isLive,
		toolConfig: data.toolConfig
	}}
/>
