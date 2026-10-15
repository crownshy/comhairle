<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowRight } from 'lucide-svelte';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { permissions } from '$lib/permissions.svelte';
	import { hasSiteAdminRole, hasSuperAdminRole } from '$lib/utils/permissions';
	import {
		conversationStatus,
		CONVERSATION_STATUS_BADGE_VARIANTS,
		CONVERSATION_STATUS_LABELS
	} from '$lib/utils/conversationStatus';
	import { workspaceLinks } from '../workspace/workspaceLinks';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let links = $derived(
		workspaceLinks({
			isSiteAdmin: hasSiteAdminRole(data.userRoles),
			isSuperAdmin: hasSuperAdminRole(data.userRoles)
		})
	);
</script>

<svelte:head>
	<title>Workspace - Comhairle Admin</title>
</svelte:head>

<!-- Workspace hrefs come from workspaceLinks, which only builds /admin section paths. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<div class="pt-page-top px-gutter flex w-full pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	<div class="flex w-full max-w-300 flex-col gap-12">
		<div class="flex flex-col items-start gap-4 sm:flex-row sm:items-center sm:justify-between">
			<h1 class="text-2xl sm:text-4xl">Workspace</h1>
			{#if permissions.can('system', 'conversation_create')}
				<NewConversationButton class="w-full sm:w-auto" />
			{/if}
		</div>

		<section class="flex flex-col gap-4" aria-labelledby="latest-conversations">
			<div class="flex items-center justify-between gap-4">
				<h2 id="latest-conversations" class="text-xl font-semibold">
					Latest conversations
				</h2>
				<a
					href={resolve('/admin/conversations')}
					class="text-primary flex items-center gap-1 text-base hover:underline"
				>
					See all
					<ArrowRight class="size-4" />
				</a>
			</div>

			{#if data.latestConversations === null}
				<p class="text-muted-foreground text-base">
					Your conversations could not be loaded.
				</p>
			{:else if data.latestConversations.length === 0}
				<p class="text-muted-foreground text-base">No conversations yet.</p>
			{:else}
				<ul
					class="bg-card border-border divide-border flex flex-col divide-y rounded-lg border"
				>
					{#each data.latestConversations as conversation (conversation.id)}
						{@const status = conversationStatus(conversation)}
						<li>
							<a
								href={resolve(
									'/(admin)/admin/conversations/[conversation_id]/configure',
									{
										conversation_id: conversation.id
									}
								)}
								class="hover:bg-muted flex items-center gap-3 px-4 py-3 text-base"
							>
								<span class="flex-1 truncate font-medium">{conversation.title}</span
								>
								{#if conversation.isShared}
									<Badge variant="outline" class="text-sm">Shared</Badge>
								{/if}
								<Badge
									variant={CONVERSATION_STATUS_BADGE_VARIANTS[status]}
									class="text-sm"
								>
									{CONVERSATION_STATUS_LABELS[status]}
								</Badge>
							</a>
						</li>
					{/each}
				</ul>
			{/if}
		</section>

		<section class="flex flex-col gap-4" aria-labelledby="workspace-links">
			<h2 id="workspace-links" class="text-xl font-semibold">Manage</h2>
			<ul class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
				{#each links as link (link.href)}
					<li>
						<a
							href={link.href}
							class="bg-card border-border hover:border-primary flex h-full items-start gap-3 rounded-lg border p-4"
						>
							<link.icon class="text-primary mt-0.5 size-5 shrink-0" />
							<span class="flex flex-col gap-1">
								<span class="text-base font-semibold">{link.label}</span>
								<span class="text-muted-foreground text-base"
									>{link.description}</span
								>
							</span>
						</a>
					</li>
				{/each}
			</ul>
		</section>
	</div>
</div>
