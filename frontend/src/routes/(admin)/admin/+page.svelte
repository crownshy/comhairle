<script lang="ts">
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let props: PageProps = $props();
	let conversations = props.data.conversations;
</script>

<div class="px-16 py-18">
	<div class="mb-10 flex flex-row items-center justify-between">
		<h1 class="text-4xl">Your conversations</h1>
		<Button variant="secondary" href="/admin/conversations/new"><Plus />New Conversation</Button
		>
	</div>
	<div class=" grid w-full grid-cols-1 gap-x-2 gap-y-5 overflow-y-auto">
		{#each conversations.records as conversation}
			<a href={`/admin/conversations/${conversation.id}/configure`}>
				<ConversationCard {conversation} />
			</a>
		{/each}
	</div>
</div>
