<script lang="ts">
	import { resolve } from '$app/paths';
	import NewConversationButton from '$lib/components/NewConversationButton.svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { permissions } from '$lib/permissions.svelte';
	import { hasSiteAdminRole, hasSuperAdminRole } from '$lib/utils/permissions';
	import type { ConversationStatusFilter } from './conversations/conversationFilters';
	import CountTile from '../workspace/CountTile.svelte';
	import HandbookCard from '../workspace/HandbookCard.svelte';
	import LiveConversationsCard from '../workspace/LiveConversationsCard.svelte';
	import PickUpCard from '../workspace/PickUpCard.svelte';
	import RecentDraftsCard from '../workspace/RecentDraftsCard.svelte';
	import WorkspaceTile from '../workspace/WorkspaceTile.svelte';
	import { workspaceLinks } from '../workspace/workspaceLinks';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	// The handbook has its own card, so it isn't repeated as a tile.
	let workspaceTiles = $derived(
		workspaceLinks({
			isSiteAdmin: hasSiteAdminRole(data.userRoles),
			isSuperAdmin: hasSuperAdminRole(data.userRoles)
		}).filter((link) => link.label !== 'Handbook')
	);

	const listHref = (status: ConversationStatusFilter) =>
		`${resolve('/admin/conversations')}?status=${status}`;

	function greeting(hour = new Date().getHours()): string {
		if (hour < 12) return 'Good morning';
		if (hour < 18) return 'Good afternoon';
		return 'Good evening';
	}
</script>

<svelte:head>
	<title>Workspace - Comhairle Admin</title>
</svelte:head>

<Tooltip.Provider>
	<div class="pt-page-top px-gutter flex w-full flex-col gap-6 pb-12 sm:pr-8 lg:pr-16">
		<div class="flex items-end justify-between gap-4">
			<div class="flex flex-col gap-1">
				<p class="text-muted-foreground text-base">
					{greeting()}{data.user?.username ? `, ${data.user.username}` : ''}
				</p>
				<h1 class="text-3xl font-semibold">Workspace</h1>
			</div>
			{#if permissions.can('system', 'conversation_create')}
				<NewConversationButton />
			{/if}
		</div>

		{#if data.pickUp}
			<PickUpCard conversation={data.pickUp} />
		{/if}

		<div class="grid auto-rows-[minmax(8rem,auto)] gap-4 md:grid-cols-4">
			<div class="md:col-span-2 md:row-span-2">
				<LiveConversationsCard
					conversations={data.live?.records ?? []}
					total={data.live?.total ?? 0}
					seeAllHref={listHref('live')}
				/>
			</div>
			<CountTile label="Drafts" count={data.drafts?.total ?? null} href={listHref('draft')} />
			<CountTile label="Completed" count={data.completedCount} href={listHref('completed')} />
			<div class="md:col-span-2">
				<RecentDraftsCard
					conversations={data.drafts?.records ?? []}
					seeAllHref={listHref('draft')}
				/>
			</div>
			<div class="md:col-span-2 md:row-span-2">
				<HandbookCard />
			</div>
			<CountTile label="Shared with you" count={data.sharedCount} href={listHref('shared')} />
			{#each workspaceTiles as link (link.href)}
				<WorkspaceTile {link} />
			{/each}
		</div>
	</div>
</Tooltip.Provider>
