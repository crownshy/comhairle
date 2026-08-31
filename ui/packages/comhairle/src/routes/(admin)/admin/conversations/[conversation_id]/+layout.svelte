<script lang="ts">
	import ConversationTabs from '$lib/components/ConversationTabs.svelte';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	import { DEFAULT_LOCALE } from '$lib/utils/constants';
	import ConversationActions from './ConversationActions.svelte';

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let displayTitle = $derived(
		getTextInLocale(
			conversation.translations?.title,
			conversation.primaryLocale ?? DEFAULT_LOCALE,
			conversation.title
		) || conversation.title
	);
</script>

<div
	class="border-border bg-background md:pl-gutter flex w-full items-center justify-between border-b py-2 pr-3 pl-14 md:pr-6"
>
	<h1 class="text-primary max-w-[22ch] truncate text-lg leading-7 font-semibold sm:max-w-[40ch]">
		{displayTitle}
	</h1>

	<ConversationActions {conversation} />
</div>

<div class="flex flex-col">
	<ConversationTabs conversationId={conversation.id} conversationIsLive={conversation.isLive} />

	{#if conversation.isComplete}
		<div class="border-destructive/20 bg-destructive/10 pl-gutter border-b py-2 pr-5">
			<p class="text-destructive text-sm">This conversation has closed</p>
		</div>
	{/if}

	<div class="bg-admin-background h-full w-full">
		{@render children()}
	</div>
</div>
