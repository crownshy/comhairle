<script>
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TeamManager from '$lib/components/TeamManager.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { notifications } from '$lib/notifications.svelte';

	const { data } = $props();
	const { streamedUsersAndPermissions } = $derived(data);
</script>

<PageHeader title="Team" description="Manage collaborators." />

<div class="border-border flex flex-col gap-4 border-t py-4 lg:flex-row lg:items-start lg:gap-6">
	<div class="flex-1">
		{#await streamedUsersAndPermissions}
			<Skeleton />
		{:then usersAndPermissions}
			{#if usersAndPermissions.err !== null}
				{notifications.addFlash({
					message: 'Failed to load users and permissions data',
					priority: 'ERROR'
				})}
			{:else}
				<TeamManager
					conversationId={data.conversation.id}
					permittedUsers={usersAndPermissions.ok}
				/>
			{/if}
		{/await}
	</div>
</div>
