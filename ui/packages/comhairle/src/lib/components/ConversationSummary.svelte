<script lang="ts">
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import type { Snippet } from 'svelte';
	import { conversationImageUrl } from '$lib/utils/conversationImage';

	type Props = {
		conversation: LocalizedConversationDto;
		children: Snippet;
	};
	let { conversation, children }: Props = $props();

	let imageUrl = $derived(conversationImageUrl(conversation.imageUrl));
</script>

<div class="mt-10 grid grid-cols-1 gap-10 md:mt-0 md:grid-cols-[400px_1fr]">
	<div class="flex flex-col gap-5">
		<h1 class="text-4xl font-semibold md:text-5xl">{conversation.title}</h1>
		<p class="text-foreground md:text-lg">
			{conversation.shortDescription}
		</p>
		<div class="hidden md:block">
			{@render children()}
		</div>
	</div>

	<div class="flex flex-col gap-5">
		<img class="max-h-117 w-full object-contain" src={imageUrl} alt={conversation.title} />
		<p class="text-foreground md:text-lg">{conversation.description}</p>
		<div class="block md:hidden">
			{@render children()}
		</div>
	</div>
</div>
