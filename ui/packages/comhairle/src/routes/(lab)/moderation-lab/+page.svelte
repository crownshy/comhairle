<script lang="ts">
	import PolisModeration from '$lib/tools/polis/PolisModeration.svelte';
	import type { RejectReason } from '$lib/moderation/moderationPolicy';
	import type { InstancedToolConfig } from '$lib/tools/types';
	import { CONVERSATION_ID, STEP_ID, WORKFLOW_ID, installMockApi } from './mockApi';

	installMockApi();

	let { data } = $props();

	// Same as the API's DEFAULT_REASONS in api/src/models/moderation_policy.rs.
	const defaultRejectReasons: RejectReason[] = [
		{
			label: 'Off-topic or unclear',
			description: 'Not about this conversation, or too unclear for people to vote on.'
		},
		{
			label: 'Harmful or abusive',
			description: 'Hate speech, threats, harassment, or content that breaks the law.'
		},
		{
			label: 'Advertising or campaigning',
			description: 'Promotes a product, service, political party or campaign.'
		},
		{
			label: 'Privacy or personal info',
			description: 'Names or identifies a private person, or shares personal details.'
		},
		{
			label: 'Duplicate',
			description: 'Makes the same point as a statement that is already in the conversation.'
		}
	];

	const toolConfig = {
		type: 'polis',
		notify_moderators: true
	} as unknown as InstancedToolConfig<'polis'>;
</script>

<svelte:head>
	<title>Moderation lab - Comhairle</title>
</svelte:head>

<div class="bg-background min-h-screen">
	<header class="border-b px-6 py-8 md:px-24">
		<p class="text-muted-foreground text-base font-medium">Engage</p>
		<h1 class="text-4xl font-bold tracking-tight">Moderate</h1>
		<p class="text-muted-foreground mt-2 text-base">
			Review and moderate what participants contribute.
		</p>
	</header>

	<main class="px-6 py-10 md:px-24">
		<PolisModeration
			conversationId={CONVERSATION_ID}
			conversationTitle="Lab conversation"
			stepName="New Polis Step"
			workflowStepId={STEP_ID}
			statements={data.statements}
			translations={[]}
			rejectReasons={defaultRejectReasons}
			{defaultRejectReasons}
			step={{
				conversationId: CONVERSATION_ID,
				workflowId: WORKFLOW_ID,
				workflowStepId: STEP_ID,
				isLive: true,
				toolConfig
			}}
		/>
	</main>
</div>
