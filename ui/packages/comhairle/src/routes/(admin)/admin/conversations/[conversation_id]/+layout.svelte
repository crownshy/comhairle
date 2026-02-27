<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Button } from '$lib/components/ui/button';
	import { MessageSquareText, ArrowUpRight, Check } from 'lucide-svelte';
	import { setContext, type Snippet } from 'svelte';
	import type { AdminPageSlots } from './slotTypes';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import { Badge } from '$lib/components/ui/badge';

	let breadcrumbContent = $state<Snippet | null>(null);
	let titleContent = $state<Snippet | null>(null);

	setContext<AdminPageSlots>('adminLayoutSlots', {
		breadcrumbContent: (content: Snippet | null) => (breadcrumbContent = content),
		titleContent: (content: Snippet | null) => (titleContent = content),
		clearTitleContent: () => (titleContent = null),
		clearBreadcrumbContent: () => (breadcrumbContent = null)
	});

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
</script>

<div class="bg-muted flex w-full flex-col justify-between gap-11 border-b-black px-16 py-8">
	<Breadcrumb.Root>
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/admin">Dashboard</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>Conversations</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Link href={`/admin/conversations/${conversation.id}/configure`}
					>{conversation.title}</Breadcrumb.Link
				>
			</Breadcrumb.Item>
			{#if breadcrumbContent}
				<Breadcrumb.Separator />
				{@render breadcrumbContent()}
			{/if}
		</Breadcrumb.List>
	</Breadcrumb.Root>
	<div class="flex w-full flex-row items-start justify-between">
		<div class="flex flex-row gap-4">
			<h2 class="text-primary flex flex-row items-center gap-2 text-2xl font-bold">
				<MessageSquareText />
				{conversation.title}
			</h2>
			<Button
				href={`/conversations/${conversation.id}/preview`}
				target="_blank"
				class="bg-blue-200 px-8 py-3 text-sm text-black"
			>
				Preview
				<ArrowUpRight />
			</Button>
			{#if conversation.isLive}
				<Button
					href={`/conversations/${conversation.id}`}
					class="bg-blue-200 px-8 py-3 text-sm text-black"
				>
					Live Conversation Link
					<ArrowUpRight />
				</Button>
			{/if}
		</div>

		<div>
			{#if conversation.isLive}
				<Badge
					variant="default"
					class="flex flex-row items-center justify-between gap-2 px-8 py-2 text-sm"
					>Launched! <Check class="text-primary size-4 rounded-full bg-white" /></Badge
				>
			{:else}
				<LaunchConversationModal conversation_id={conversation.id} />
			{/if}
		</div>
	</div>
	<div class="flex w-full flex-row items-center justify-between">
		{#if titleContent}
			{@render titleContent()}
		{/if}
	</div>
</div>
<div class="bg-muted flex-grow px-16 py-18">
	{@render children()}
</div>
