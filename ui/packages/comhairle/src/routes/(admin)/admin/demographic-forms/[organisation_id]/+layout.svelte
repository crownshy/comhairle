<script lang="ts">
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabStripItem from '$lib/components/TabStripItem.svelte';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import { resolve } from '$app/paths';
	import { kebabToSentenceCase } from '$lib/utils/casingUtils';

	const { data, params, children } = $props();

	const tabs = ['forms', 'questions'] as const;
</script>

<SectionHeader
	hasTabs
	content={{
		eyebrow: 'Forms',
		title: 'Demographic forms',
		description:
			'Build questions, put them together into forms, then add a form to any conversation as a demographic step.'
	}}
>
	{#snippet actions()}
		{#if data.canEdit}
			<Button>
				<Plus class="size-4" />New form
			</Button>
		{/if}
	{/snippet}
</SectionHeader>

<TabStripShell ariaLabel="Demographics section">
	{#each tabs as tab (tab)}
		<TabStripItem
			href={resolve(`/(admin)/admin/demographic-forms/[organisation_id]/${tab}`, {
				organisation_id: params.organisation_id
			})}
			isActive={(pathname) => pathname.endsWith(tab)}
		>
			{kebabToSentenceCase(tab)}
		</TabStripItem>
	{/each}
</TabStripShell>

{@render children()}
