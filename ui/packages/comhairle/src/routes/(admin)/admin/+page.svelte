<script lang="ts">
	import ConversationCard from '$lib/components/ConversationCard.svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import type { PageProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Home } from 'lucide-svelte';

	let props: PageProps = $props();
	let ownedConversations = $derived(props.data.ownedConversations?.records ?? []);
	let permittedConversations = $derived(props.data.permittedConversations?.records ?? []);
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
				<Breadcrumb.Link href="/admin">Workspace</Breadcrumb.Link>
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
		<NewConversationButton class="w-full sm:w-auto" label="Create New Conversation" />
	</div>
	<div class="flex w-full flex-col gap-11 overflow-y-auto">
		<section class="flex flex-col gap-6">
			<h2 class="text-muted-foreground text-base font-medium">Owned Conversations</h2>
			<div class="grid w-full grid-cols-1 gap-x-2 gap-y-16">
				{#each ownedConversations as conversation (conversation.id)}
					<ConversationCard {conversation} />
				{/each}
			</div>
		</section>

		{#if permittedConversations.length > 0}
			<section class="flex flex-col gap-6">
				<h2 class="text-muted-foreground text-base font-medium">Permitted Conversations</h2>
				<div class="grid w-full grid-cols-1 gap-x-2 gap-y-16">
					{#each permittedConversations as conversation (conversation.id)}
						<ConversationCard {conversation} />
					{/each}
				</div>
			</section>
		{/if}
	</div>
</div>
