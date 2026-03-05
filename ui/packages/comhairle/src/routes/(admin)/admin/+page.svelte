<script lang="ts">
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Home } from 'lucide-svelte';

	let props: PageProps = $props();
	let conversations = props.data.conversations;
</script>

<svelte:head>
	<title>Admin Dashboard - Comhairle</title>
</svelte:head>

<div
	class="bg-muted flex w-full flex-col justify-between gap-11 border-b-black px-4 py-6 sm:px-8 md:px-16 md:py-8"
>
	<Breadcrumb.Root>
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/admin">Dashboard</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>Conversations</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>

	<div class="mb-10 flex flex-col items-start gap-4 lg:flex-row lg:justify-between">
		<div class="flex items-center gap-2">
			<Home class="size-7 sm:size-9" />
			<h1 class="text-2xl sm:text-4xl">Your conversations</h1>
		</div>
		<Button class="w-full sm:w-auto" variant="default" href="/admin/conversations/new"
			><Plus />Create New Conversation</Button
		>
	</div>
	<div class="grid w-full grid-cols-1 gap-x-2 gap-y-16 overflow-y-auto">
		{#each conversations.records as conversation (conversation.id)}
			<a href={`/admin/conversations/${conversation.id}/configure`}>
				<ConversationCard {conversation} />
			</a>
		{/each}
	</div>
</div>
