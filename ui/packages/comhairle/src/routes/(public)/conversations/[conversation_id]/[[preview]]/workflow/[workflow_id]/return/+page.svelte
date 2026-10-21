<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages';
	import type { PageProps } from './$types';
	import RevisitStepCards from '../RevisitStepCards.svelte';

	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>
		{m.conversations_return_page_title({ conversation: data.conversation.title })} - Comhairle
	</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-5xl flex-col gap-8 px-4 py-10 sm:px-6 md:py-16">
	{#if data.loadFailed}
		<div class="flex flex-col gap-3">
			<h1 class="text-3xl font-bold">{m.something_went_wrong()}</h1>
			<p class="text-base md:text-lg">{m.conversations_return_error_message()}</p>
			<a class="text-primary self-start text-base underline" href={resolve('/conversations')}>
				{m.conversations_return_error_link()}
			</a>
		</div>
	{:else}
		<header class="flex flex-col gap-3">
			<h1 class="text-3xl leading-tight font-bold md:text-4xl">
				{m.conversations_return_heading()}
			</h1>
			<p class="text-base md:text-lg">{m.conversations_return_message()}</p>
		</header>
		<RevisitStepCards
			steps={data.revisitableSteps}
			conversationId={data.conversation.id}
			workflowId={data.workflowId}
			isPreview={data.isPreview}
		/>
	{/if}
</div>
