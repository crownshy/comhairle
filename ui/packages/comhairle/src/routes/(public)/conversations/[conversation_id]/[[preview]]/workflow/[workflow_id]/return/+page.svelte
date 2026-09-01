<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import type { PageProps } from './$types';
	import JourneyRail from './JourneyRail.svelte';
	import * as m from '$lib/paraglide/messages';

	const { data }: PageProps = $props();

	let pageTitle = $derived(
		`Jump back in${data.conversation ? ` to ${data.conversation.title}` : ''} - Comhairle`
	);
</script>

<svelte:head>
	<title>{pageTitle}</title>
</svelte:head>

<!-- Branching on the conversation rather than on `error`: the load returns one shape or the
	other, and this is the half the page needs. -->
{#if data.conversation}
	<JourneyRail
		conversation={data.conversation}
		steps={data.steps}
		workflowId={data.workflowId}
		preview={data.preview}
		hasKnowledgeBaseDocs={data.hasKnowledgeBaseDocs}
		queryString={page.url.search}
	/>
{:else}
	<div class="mx-auto flex w-full max-w-2xl flex-col items-center gap-4 px-5 py-20 md:px-6">
		<h1 class="text-2xl font-bold">{m.something_went_wrong()}</h1>
		<p class="text-base">{m.conversations_return_error_message()}</p>
		<a class="text-base underline underline-offset-4" href={resolve('/(public)/conversations')}>
			{m.conversations_return_error_link()}
		</a>
	</div>
{/if}
