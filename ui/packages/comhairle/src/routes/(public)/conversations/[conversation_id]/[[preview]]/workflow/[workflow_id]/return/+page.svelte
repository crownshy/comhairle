<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();

	const { revisitableSteps, conversation, workflowId, error } = data;

	const pageTitle = $derived(`Jump back in${conversation ? ` to ${conversation.title}` : ''}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<div class="flex flex-col items-center gap-4 sm:py-2 md:py-10">
	{#if error}
		<h1 class="text-2xl font-bold">Something went wrong</h1>
		<p>We were unable to load your progress for this conversation.</p>
		<p>Return to the <a class="underline" href="/conversations">conversations page?</a></p>
	{:else}
		<h1 class="text-2xl font-bold">You have completed this conversation!</h1>
		<p>Jump back in at one of the revisitable steps below.</p>
		<ul class="mt-4 flex flex-col gap-3">
			{#each revisitableSteps as step (step.id)}
				<li
					class="hover:bg-primary hover:text-background rounded-xl border px-8 py-2 transition-all duration-300 ease-in-out"
				>
					<a
						href={`/conversations/${conversation.id}/workflow/${workflowId}/s/${step.id}`}
						class="text-center"
					>
						<ContentRenderer content={step.name} class="font-bold [&_p]:text-lg" />
						<ContentRenderer content={step.description} />
					</a>
				</li>
			{/each}
		</ul>
	{/if}
</div>
