<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TabContent from '../TabContent.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { capitalise } from '$lib/utils/casingUtils';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';

	const { params, children } = $props();

	const tabs = ['email', 'open-links', 'physical'] as const;
</script>

<svelte:head>
	<title>Manage Invites - Comhairle Admin</title>
</svelte:head>

{#snippet Tab(id: (typeof tabs)[number])}
	{@const active = page.route.id?.endsWith(id)}
	<li>
		<a
			href={resolve(`/(admin)/admin/conversations/[conversation_id]/invites/${id}`, {
				conversation_id: params.conversation_id
			})}
			data-sveltekit-noscroll
			class="text-foreground inline-flex h-9 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
			class:text-primary={active}
			class:opacity-70={!active}
			class:hover:opacity-100={!active}
			aria-current={active ? 'page' : undefined}
		>
			{id.split('-').reduce((acc, word) => acc.concat(' ', capitalise(word)), '')}
		</a>
	</li>
{/snippet}

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab)}
		{@render Tab(tab)}
	{/each}
</TabStripShell>

<TabContent>
	<PageHeader title="Recruit" />
	{@render children()}
</TabContent>
