<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { PageProps } from './$types';
	import * as m from '$lib/paraglide/messages';

	const { data }: PageProps = $props();

	const { revisitableSteps, conversation, workflowId, error } = data;

	const pageTitle = $derived(`Jump back in${conversation ? ` to ${conversation.title}` : ''}`);
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<div class="flex flex-col items-center gap-4 sm:py-2 md:py-10">
	{#if error}
		<h1 class="text-2xl font-bold">{m.something_went_wrong()}</h1>
		<p>{m.conversations_return_error_message()}</p>
		<a class="underline" href="/conversations">{m.conversations_return_error_link()}</a>
	{:else}
		<h1 class="text-2xl font-bold">{m.conversations_return_heading()}</h1>
		<p>{m.conversations_return_message()}</p>
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
