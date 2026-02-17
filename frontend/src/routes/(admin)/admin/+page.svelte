<script lang="ts">
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';

	let props: PageProps = $props();
	let conversations = props.data.conversations;
</script>

<div class="flex w-full flex-col justify-between gap-11 border-b-black bg-white px-16 py-8">
	
	<Breadcrumb.Root>
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/admin">Dashboard</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>Conversations</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>

	<div class="mb-10 flex flex-row items-center justify-between">
		<h1 class="text-4xl">Your conversations</h1>
		<Button variant="default" href="/admin/conversations/new"><Plus />Create New Conversation</Button
		>
	</div>
	<div class=" grid w-full grid-cols-1 gap-x-2 gap-y-5 overflow-y-auto">
		{#each conversations.records as conversation (conversation.id)}
			<a href={`/admin/conversations/${conversation.id}/configure`}>
				<ConversationCard {conversation} />
			</a>
		{/each}
	</div>
</div>
