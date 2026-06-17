<script lang="ts">
	import { setContext, type Snippet } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ArrowUpRight, MoreHorizontal, Eye, ExternalLink, Check, CircleX } from 'lucide-svelte';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';
	import ConversationTabs from '$lib/components/ConversationTabs.svelte';
	import {
		CONVERSATION_TAB_EXTRAS_CTX,
		type ConversationTabExtras
	} from '$lib/conversationTabExtras';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';

	let { data, children } = $props();

	let tabExtras = $state<ConversationTabExtras>({ primary: null, secondary: null });
	setContext(CONVERSATION_TAB_EXTRAS_CTX, tabExtras);

	let conversation = $derived(data.conversation);
	let displayTitle = $derived(
		getTextInLocale(
			conversation.translations?.title,
			conversation.primaryLocale ?? 'en',
			conversation.title
		) || conversation.title
	);
	let endModalOpen = $state(false);
	let launchModalOpen = $state(false);
</script>

<!-- Row 1: conversation title + launch controls -->
<div
	class="border-border bg-background flex w-full items-center justify-between border-b py-2 pr-3 pl-14 md:px-6"
>
	<h1
		class="text-primary max-w-[22ch] truncate text-lg leading-7 font-semibold sm:max-w-[40ch]"
		title={displayTitle}
	>
		{displayTitle}
	</h1>

	<!-- Mobile actions: single more-menu -->
	<div class="flex shrink-0 items-center lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				class="bg-primary/20 hover:bg-primary/30 inline-flex size-9 items-center justify-center rounded-full"
				aria-label="Actions"
			>
				<MoreHorizontal class="size-4" />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end" class="w-56">
				<DropdownMenu.Item>
					<a
						href={`/conversations/${conversation.id}/preview`}
						target="_blank"
						class="flex w-full items-center gap-2"
					>
						<Eye class="size-4" />
						Preview
					</a>
				</DropdownMenu.Item>
				{#if conversation.isLive}
					<DropdownMenu.Item>
						<a
							href={`/conversations/${conversation.id}`}
							class="flex w-full items-center gap-2"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					{#if !conversation.isComplete}
						<DropdownMenu.Item
							class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
							onclick={() => (endModalOpen = true)}
						>
							<CircleX class="text-destructive size-4" />
							End Conversation
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => (endModalOpen = true)}>
							<Check class="size-4" />
							Re-open Conversation
						</DropdownMenu.Item>
					{/if}
				{:else}
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={() => (launchModalOpen = true)}>
						<ArrowUpRight class="size-4" />
						Launch Conversation
					</DropdownMenu.Item>
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>

	<!-- Desktop actions -->
	<div class="hidden shrink-0 items-center gap-4 md:flex">
		<Button
			href={`/conversations/${conversation.id}/preview`}
			target="_blank"
			variant="secondary"
			class="bg-primary/20 text-foreground hover:bg-primary/30 inline-flex h-10 rounded-full px-4 text-sm"
		>
			Preview
			<ArrowUpRight class="size-4" />
		</Button>

		{#if conversation.isLive}
			{#if !conversation.isComplete}
				<span
					class="bg-primary text-primary-foreground inline-flex h-10 items-center gap-2 rounded-full py-1 pr-1 pl-5 text-sm font-medium"
				>
					Launched
					<span
						class="bg-primary-foreground text-primary inline-flex size-8 items-center justify-center rounded-full"
					>
						<Check class="size-4" strokeWidth={3} />
					</span>
				</span>
			{/if}

			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="bg-primary/20 hover:bg-primary/30 inline-flex size-10 items-center justify-center rounded-full"
					aria-label="More actions"
				>
					<MoreHorizontal class="size-4" />
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					<DropdownMenu.Item>
						<a
							href={`/conversations/${conversation.id}`}
							class="flex w-full items-center gap-2"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					</DropdownMenu.Item>
					<DropdownMenu.Separator />
					{#if !conversation.isComplete}
						<DropdownMenu.Item
							class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
							onclick={() => (endModalOpen = true)}
						>
							<CircleX class="text-destructive size-4" />
							End Conversation
						</DropdownMenu.Item>
					{:else}
						<DropdownMenu.Item onclick={() => (endModalOpen = true)}>
							<Check class="size-4" />
							Re-open Conversation
						</DropdownMenu.Item>
					{/if}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		{:else}
			<Button variant="default" class="h-[40px]" onclick={() => (launchModalOpen = true)}>
				Launch Conversation
			</Button>
		{/if}
	</div>
</div>

<!-- Modals (triggered programmatically from mobile dropdown or desktop UI) -->
<EndConversationModal {conversation} bind:open={endModalOpen} hideTrigger />
<LaunchConversationModal
	conversation_id={conversation.id}
	bind:open={launchModalOpen}
	hideTrigger
/>

<!-- Row 2: section tabs -->
<ConversationTabs conversationId={conversation.id} conversationIsLive={conversation.isLive} />

<!-- Row 3+ : section-specific sub-strips injected via context (e.g. workflow steps, sub-tabs) -->
{#if tabExtras.primary}
	{@render tabExtras.primary()}
{/if}
{#if tabExtras.secondary}
	{@render tabExtras.secondary()}
{/if}

{#if conversation.isComplete}
	<div class="border-destructive/20 bg-destructive/10 border-b px-5 py-2">
		<p class="text-destructive text-sm">This conversation has closed</p>
	</div>
{/if}

<div class="bg-muted grow px-4 py-8 sm:px-8 sm:pb-12 md:py-10 lg:px-16 lg:pb-18">
	<div class="mx-auto h-full w-full max-w-[1200px]">
		{@render children()}
	</div>
</div>
