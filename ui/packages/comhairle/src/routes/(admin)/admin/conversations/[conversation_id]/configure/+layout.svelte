<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { capitalise } from '$lib/utils/casingUtils';

	let { data, children, params } = $props();

	const tabs = ['details', 'content', 'glossary', 'access'] as const;
</script>

{#snippet Tab(id: (typeof tabs)[number] | 'team')}
	{@const active = page.route.id?.endsWith(id)}
	<li>
		<a
			href={resolve(`/(admin)/admin/conversations/[conversation_id]/configure/${id}`, {
				conversation_id: params.conversation_id
			})}
			data-sveltekit-noscroll
			class="text-foreground inline-flex h-9 items-center px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
			class:text-primary={active}
			class:opacity-70={!active}
			class:hover:opacity-100={!active}
			aria-current={active ? 'page' : undefined}
		>
			{capitalise(id)}
		</a>
	</li>
{/snippet}

<TabStripShell ariaLabel="Configure sections">
	{#each tabs as tab (tab)}
		{@render Tab(tab)}
	{/each}
	{#if data.isConversationOwner}
		{@render Tab('team')}
	{/if}
</TabStripShell>

<section class="pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	{@render children?.()}
</section>
