<script lang="ts">
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import type { PageProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Home } from 'lucide-svelte';
	import { m } from '$lib/paraglide/messages';
	import { permissions } from '$lib/permissions.svelte';
	import StatusFilterChips from './StatusFilterChips.svelte';
	import { statusFilterLabels } from '$lib/utils/conversationFilters';

	let props: PageProps = $props();
	let statusFilter = $derived(props.data.filter);
	let visibleConversations = $derived(props.data.conversations);
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
				<Breadcrumb.Link href="/admin">{m.workspace()}</Breadcrumb.Link>
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
		{#if permissions.can('system', 'conversation_create')}
			<NewConversationButton class="w-full sm:w-auto" label="Create New Conversation" />
		{/if}
	</div>
	<div class="flex w-full flex-col gap-11 overflow-y-auto">
		<StatusFilterChips active={statusFilter} />

		{#if visibleConversations.length > 0}
			<div class="grid w-full grid-cols-1 gap-x-2 gap-y-16">
				{#each visibleConversations as conversation (conversation.id)}
					<ConversationCard {conversation} />
				{/each}
			</div>
		{:else if statusFilter === 'all'}
			<p class="text-muted-foreground text-base">You don't have any conversations yet.</p>
		{:else}
			<p class="text-muted-foreground text-base">
				No {statusFilterLabels[statusFilter].toLowerCase()} conversations.
			</p>
		{/if}
	</div>
</div>
