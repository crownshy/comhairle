<script lang="ts">
	import ConversationSupportSidebar from '$lib/components/ConversationSupportSidebar.svelte';
	import { page } from '$app/state';
	import type { LayoutProps } from './$types';

	let { data, children }: LayoutProps = $props();
	let { conversation, hasKnowledgeBaseDocs, availableDocuments, workflowSteps } = $derived(data);

	// The support sidebar sits at the layout, above the per-step route, so it does not otherwise
	// know which step the participant is on. Derive the active learn step's title from the route
	// param and pass it as a retrieval hint for the assistant (see ADR-0010). Non-learn steps get
	// no hint, so the assistant behaves as before there.
	let currentStep = $derived(
		workflowSteps?.find((step) => step.id === page.params.workflow_step_id)
	);
	let currentStepTitle = $derived(
		currentStep?.toolConfig?.type === 'learn' ? currentStep.name : undefined
	);
</script>

{@render children()}

<ConversationSupportSidebar
	{conversation}
	{hasKnowledgeBaseDocs}
	{availableDocuments}
	{currentStepTitle}
/>
