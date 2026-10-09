<script lang="ts">
	import { resolve } from '$app/paths';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { kebabToSentenceCase } from '$lib/utils/casingUtils';
	import TabContent from '../TabContent.svelte';
	import { permissions } from '$lib/permissions.svelte';

	let { data, children, params } = $props();
	const canAdmin = $derived(
		permissions.can('conversation', 'conversation_admin', data.conversation.id)
	);

	const tabs = ['details', 'content', 'glossary', 'moderation-policy', 'access'] as const;
</script>

{#snippet Tab(tab: (typeof tabs)[number] | 'team')}
	<TabStripItem
		href={resolve(`/(admin)/admin/conversations/[conversation_id]/configure/${tab}`, {
			conversation_id: params.conversation_id
		})}
		isActive={(pathname) => pathname.endsWith(tab)}
	>
		{kebabToSentenceCase(tab)}
	</TabStripItem>}
{/snippet}

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab)}
		{@render Tab(tab)}
	{/each}
	{#if canAdmin}
		{@render Tab('team')}
	{/if}
</TabStripShell>

<TabContent>
	{@render children?.()}
</TabContent>
