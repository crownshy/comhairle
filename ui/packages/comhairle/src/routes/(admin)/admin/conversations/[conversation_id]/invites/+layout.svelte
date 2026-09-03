<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TabContent from '../TabContent.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import { resolve } from '$app/paths';
	import TabStripItem from '$lib/components/TabStripItem.svelte';

	const { params, children } = $props();

	const tabs = ['email', 'open-links'] as const;
</script>

<svelte:head>
	<title>Manage Invites - Comhairle Admin</title>
</svelte:head>

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab)}
		<TabStripItem
			{tab}
			href={resolve(`/(admin)/admin/conversations/[conversation_id]/invites/${tab}`, {
				conversation_id: params.conversation_id
			})}
			label={snakeToSentenceCase(tab)}
		/>
	{/each}
</TabStripShell>

<TabContent>
	<PageHeader title="Recruit" />
	{@render children()}
</TabContent>
