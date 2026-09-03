<script lang="ts">
	import { resolve } from '$app/paths';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { capitalise } from '$lib/utils/casingUtils';
	import TabContent from '../TabContent.svelte';

	let { data, children, params } = $props();

	const tabs = ['details', 'content', 'glossary', 'access'] as const;
</script>

{#snippet Tab(tab: (typeof tabs)[number] | 'team')}
	<TabStripItem
		{tab}
		href={resolve(`/(admin)/admin/conversations/[conversation_id]/configure/${tab}`, {
			conversation_id: params.conversation_id
		})}
	>
		{capitalise(tab)}
	</TabStripItem>
{/snippet}

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab)}
		{@render Tab(tab)}
	{/each}
	{#if data.isConversationOwner}
		{@render Tab('team')}
	{/if}
</TabStripShell>

<TabContent>
	{@render children?.()}
</TabContent>
