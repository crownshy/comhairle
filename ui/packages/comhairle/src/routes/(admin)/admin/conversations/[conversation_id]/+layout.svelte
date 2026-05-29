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
	import type { AdminPageSlots, BreadcrumbCrumb } from './slotTypes';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';

	let breadcrumbTrail = $state<BreadcrumbCrumb[] | null>(null);
	let titleContent = $state<Snippet | null>(null);

	setContext<AdminPageSlots>('adminLayoutSlots', {
		breadcrumbTrail: (trail: BreadcrumbCrumb[] | null) => (breadcrumbTrail = trail),
		titleContent: (content: Snippet | null) => (titleContent = content),
		clearTitleContent: () => (titleContent = null),
		clearBreadcrumbTrail: () => (breadcrumbTrail = null)
	});

	let { data, children } = $props();

	let conversation = $derived(data.conversation);
	let endModalOpen = $state(false);

	let allCrumbs = $derived<BreadcrumbCrumb[]>([
		{ label: 'Workspace', href: '/admin' },
		{ label: 'Conversations' },
		{
			label: conversation.title,
			href: `/admin/conversations/${conversation.id}/configure`
		},
		...(breadcrumbTrail ?? [])
	]);
	let currentCrumb = $derived(allCrumbs[allCrumbs.length - 1]);
	let parentCrumbs = $derived(allCrumbs.slice(0, -1));
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
					<DropdownMenu.Root>
						<DropdownMenu.Trigger
							class="hover:text-foreground focus-visible:ring-ring text-muted-foreground flex size-7 items-center justify-center rounded-md focus-visible:ring-1 focus-visible:outline-none"
							aria-label="Show breadcrumb trail"
						>
							<Breadcrumb.Ellipsis class="size-4" />
						</DropdownMenu.Trigger>
						<DropdownMenu.Content align="start">
							{#each parentCrumbs as crumb (crumb.label)}
								{#if crumb.href}
									<DropdownMenu.Item>
										{#snippet child({ props })}
											<a {...props} href={crumb.href}>{crumb.label}</a>
										{/snippet}
									</DropdownMenu.Item>
								{:else}
									<DropdownMenu.Item disabled>{crumb.label}</DropdownMenu.Item>
								{/if}
							{/each}
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</Breadcrumb.Item>
				<Breadcrumb.Separator class="shrink-0" />
				<Breadcrumb.Item class="min-w-0">
					<Breadcrumb.Page
						class="block max-w-[20ch] truncate sm:max-w-[32ch] lg:max-w-[40ch]"
						title={currentCrumb.label}
					>
						{currentCrumb.label}
					</Breadcrumb.Page>
				</Breadcrumb.Item>
			</Breadcrumb.List>
		</Breadcrumb.Root>
		<div class="flex w-full min-w-0 items-center gap-3 xl:ml-auto xl:w-auto">
			<!-- Conversation Title -->
			<div class="flex min-w-0 shrink items-center gap-2">
				<MessageSquareText class="text-primary size-5 shrink-0" />
				<span
					class="text-primary max-w-[30ch] truncate text-base font-semibold sm:max-w-[40ch] sm:text-lg lg:max-w-[60ch] xl:max-w-[24ch]"
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
			class="flex w-full max-w-[1200px] min-w-0 flex-col items-start justify-between gap-4 lg:flex-row lg:items-center"
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
	<div class="mx-auto h-full w-full max-w-[1200px]">
		{@render children()}
	</div>
</div>
