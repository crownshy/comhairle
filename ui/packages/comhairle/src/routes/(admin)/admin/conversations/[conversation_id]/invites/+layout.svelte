<script lang="ts">
	import { resolve } from '$app/paths';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import TabContent from '../TabContent.svelte';

	const { children, params } = $props();

	const slugs = ['email', 'open-links'] as const;
</script>

<svelte:head>
	<title>Manage Invites - Comhairle Admin</title>
</svelte:head>

<TabStripShell>
	{#each slugs as slug (slug)}
		<TabStripItem
			href={resolve(`/(admin)/admin/conversations/[conversation_id]/invites/${slug}`, {
				conversation_id: params.conversation_id
			})}
			isActive={(pathname) => pathname.endsWith(slug)}
		>
			{snakeToSentenceCase(slug)}
		</TabStripItem>
	{/each}
</TabStripShell>

<TabContent>
	<PageHeader title="Recruit" />
	{@render children()}
</TabContent>
