<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import {
		MessageSquareText,
		ArrowUpRight,
		MoreHorizontal,
		Eye,
		ExternalLink,
		Check,
		CircleX
	} from 'lucide-svelte';
	import { setContext, type Snippet } from 'svelte';
	import type { AdminPageSlots } from './slotTypes';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';

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
	let endModalOpen = $state(false);
</script>

<!-- Top bar: breadcrumb + conversation name + launch controls -->
<div
	class="border-base-border bg-background flex w-full flex-col items-center border-b px-4 pt-10 pb-6 sm:px-8 lg:px-16"
>
	<!--
		- xl+: breadcrumb, conversation name, and actions all share one row.
		- below xl: breadcrumb on its own row
	-->
	<div class="flex w-full max-w-[1200px] flex-wrap items-center gap-x-4 gap-y-3">
		<Breadcrumb.Root class="min-w-0 xl:max-w-[50%]">
			<Breadcrumb.List class="flex-nowrap">
				<Breadcrumb.Item class="shrink-0">
					<Breadcrumb.Link href="/admin">Workspace</Breadcrumb.Link>
				</Breadcrumb.Item>
				<Breadcrumb.Separator class="shrink-0" />
				<Breadcrumb.Item class="shrink-0">Conversations</Breadcrumb.Item>
				<Breadcrumb.Separator class="shrink-0" />
				<Breadcrumb.Item class="min-w-0">
					<Breadcrumb.Link
						href={`/admin/conversations/${conversation.id}/configure`}
						class="block max-w-[12ch] truncate sm:max-w-[20ch]"
						title={conversation.title}
					>
						{conversation.title}
					</Breadcrumb.Link>
				</Breadcrumb.Item>
				{#if breadcrumbContent}
					<Breadcrumb.Separator class="shrink-0" />
					{@render breadcrumbContent()}
				{/if}
			</Breadcrumb.List>
		</Breadcrumb.Root>
		<div class="flex w-full min-w-0 items-center gap-3 xl:ml-auto xl:w-auto">
			<!-- Identity -->
			<div class="flex min-w-0 shrink items-center gap-2">
				<MessageSquareText class="text-primary size-5 shrink-0" />
				<span
					class="text-primary truncate text-base font-semibold sm:text-lg"
					title={conversation.title}
				>
					{conversation.title}
				</span>
			</div>

			<!-- Actions -->
			<div class="ml-auto flex shrink-0 items-center gap-2">
				{#if conversation.isLive}
					<!--
						Live state: compact "Launched" pill + 3-dot menu containing
						Preview, Live Link, and End Conversation (obscured/destructive).
					-->
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
									href={`/conversations/${conversation.id}/preview`}
									target="_blank"
									class="flex w-full items-center gap-2"
								>
									<Eye class="size-4" />
									Preview
								</a>
							</DropdownMenu.Item>
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
							<DropdownMenu.Item
								class="text-destructive focus:text-destructive focus:bg-destructive/10 hover:text-destructive! hover:bg-destructive/20!"
								onclick={() => (endModalOpen = true)}
							>
								<CircleX class="text-destructive size-4" />
								End Conversation
							</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>

					<EndConversationModal {conversation} bind:open={endModalOpen} hideTrigger />
				{:else}
					<!--
						Pre-launch state
					-->
					<Button
						href={`/conversations/${conversation.id}/preview`}
						target="_blank"
						variant="secondary"
						class="bg-primary/20 text-foreground hover:bg-primary/30 inline-flex h-10 rounded-full px-4 text-sm"
					>
						Preview
						<ArrowUpRight class="size-4" />
					</Button>

					<LaunchConversationModal conversation_id={conversation.id} />
				{/if}
			</div>
		</div>
	</div>
</div>

<!-- Secondary bar: page title + prev/next navigation -->
{#if titleContent}
	<div
		class="border-base-border bg-background flex w-full flex-col items-center border-b px-4 py-6 sm:px-8 lg:px-16"
	>
		<div
			class="flex w-full max-w-[1200px] flex-col items-start justify-between gap-4 lg:flex-row lg:items-center"
		>
			{@render titleContent()}
		</div>
		{#if conversation.isComplete}
			<div class="mt-2 w-full max-w-[1200px]">
				<p class="text-sm text-red-500">This conversation has closed</p>
			</div>
		{/if}
	</div>
{/if}

<div class="bg-muted grow px-4 py-8 sm:px-8 sm:pt-10 sm:pb-12 lg:px-16 lg:pb-18">
	<div class="mx-auto w-full max-w-[1200px]">
		{@render children()}
	</div>
</div>
