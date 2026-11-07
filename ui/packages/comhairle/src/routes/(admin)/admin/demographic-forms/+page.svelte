<script lang="ts">
	import { resolve } from '$app/paths';
	import Callout from '$lib/components/Callout.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { ChevronRight } from 'lucide-svelte';

	const { data } = $props();
</script>

<SectionHeader
	content={{
		eyebrow: 'Forms',
		title: 'Demographic forms',
		description:
			'Build questions, put them together into forms, then add a form to any conversation as a demographic step.'
	}}
/>

<TabContent>
	{#if data.userOrganisations.length === 0}
		<Callout variant="warning" class="px-7 py-5">
			<h2 class="text-lg font-semibold">No organisations yet</h2>

			<p class="text-muted-foreground text-base">
				You are not a member of an organisation, so there are no demographic forms to manage
				yet.
			</p>
		</Callout>
	{:else}
		<h2 class="text-muted-foreground text-l font-semibold">Select an organisation</h2>

		<section class="pt-3">
			{#each data.userOrganisations as userOrganisation (userOrganisation.organization.id)}
				<a
					href={resolve('/(admin)/admin/demographic-forms/[organisation_id]', {
						organisation_id: userOrganisation.organization.id
					})}
					class="border-border bg-card hover:bg-accent/30 block border px-5 py-4 first:rounded-t-xl last:rounded-b-xl"
				>
					<div class="flex flex-row items-center justify-between pr-5">
						<div class="flex flex-col">
							<span class="text-xl font-medium">
								{userOrganisation.organization.name}
							</span>
							<span class="text-muted-foreground">
								{userOrganisation.organization.description}
							</span>
						</div>
						<ChevronRight class="size-8" strokeWidth={1.2} />
					</div>
				</a>
			{/each}
		</section>
	{/if}
</TabContent>
