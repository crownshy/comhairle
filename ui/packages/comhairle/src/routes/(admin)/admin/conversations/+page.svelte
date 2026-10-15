<script lang="ts">
	import { Home } from 'lucide-svelte';
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { m } from '$lib/paraglide/messages';
	import { permissions } from '$lib/permissions.svelte';
	import type { PageProps } from './$types';
	import { CONVERSATION_STATUS_FILTER_LABELS } from './conversationFilters';
	import StatusFilterChips from './StatusFilterChips.svelte';

	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>Admin Dashboard - Comhairle</title>
</svelte:head>

<div class="bg-muted pt-page-top px-gutter flex w-full pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	<div class="@container flex w-full max-w-300 flex-col gap-11">
		<Breadcrumb.Root>
			<Breadcrumb.List>
				<Breadcrumb.Item>
					<Breadcrumb.Link href="/admin">{m.workspace()}</Breadcrumb.Link>
				</Breadcrumb.Item>
				<Breadcrumb.Separator />
				<Breadcrumb.Item>Conversations</Breadcrumb.Item>
			</Breadcrumb.List>
		</Breadcrumb.Root>

		<!-- The right padding matches the card padding so the button lines up with the card images. -->
		<div
			class="mb-10 flex flex-col items-start gap-4 @4xl:flex-row @4xl:items-center @4xl:justify-between @4xl:pr-6"
		>
			<div class="flex items-center gap-2">
				<Home class="size-7 sm:size-9" />
				<h1 class="text-2xl sm:text-4xl">Your conversations</h1>
			</div>
			{#if permissions.can('system', 'conversation_create')}
				<NewConversationButton class="w-full sm:w-auto" label="Create New Conversation" />
			{/if}
		</div>
		<div class="flex w-full flex-col gap-11 overflow-y-auto">
			<StatusFilterChips active={data.filter} />

			<div class="grid w-full grid-cols-1 gap-x-2 gap-y-16">
				{#each data.conversations as conversation (conversation.id)}
					<ConversationCard
						{conversation}
						isShared={data.sharedConversationIds.has(conversation.id)}
					/>
				{:else}
					<p class="text-muted-foreground text-base">
						{#if data.filter === 'all'}
							You don't have any conversations yet.
						{:else}
							No {CONVERSATION_STATUS_FILTER_LABELS[data.filter].toLowerCase()} conversations.
						{/if}
					</p>
				{/each}
			</div>
		</div>
	</div>
</div>
