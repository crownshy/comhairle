<script lang="ts">
	import type { PageProps } from '../$types.js';
	let { data }: PageProps = $props();
	let { conversation, workflows } = data;

	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	let firstWorkflow = $derived(workflows[0]);

	let firstWorkflowPath = $derived(
		`/conversations/${conversation.slug}/workflow/${firstWorkflow.id}/next`
	);
</script>

<div class="mx-10 mt-20 md:mx-auto md:max-w-3/5">
	{#if conversation.faqs}
		<ContentRenderer content={conversation.faqs} />
	{/if}
	<Button href={firstWorkflowPath}>Resume taking part</Button>
</div>
