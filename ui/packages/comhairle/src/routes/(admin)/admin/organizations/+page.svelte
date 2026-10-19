<script lang="ts">
	import type { PageProps } from './$types';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus } from 'lucide-svelte';
	import { permissions } from '$lib/permissions.svelte';

	let props: PageProps = $props();
	let organizationAccess = $derived(props.data.userOrganizations?.organizations ?? []);
	let canCreateOrganization = $derived(permissions.can('system', 'organization_create'));
</script>

<svelte:head>
	<title>Organisations - Comhairle Admin</title>
</svelte:head>

<section class="pt-page-top px-gutter max-w-300 grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	<PageHeader
		title="Your organisations"
		description="Organisations you belong to. Open one to see its details and manage its team."
	>
		{#snippet actions()}
			{#if canCreateOrganization}
				<Button href="/admin/organizations/new">
					<Plus class="size-4" />
					Create organisation
				</Button>
			{/if}
		{/snippet}
	</PageHeader>

	{#if organizationAccess.length === 0}
		<div class="bg-card border-border rounded-2xl border p-6">
			<p class="text-base">You are not a member of any organisations yet.</p>
		</div>
	{:else}
		<div class="grid w-full grid-cols-1 gap-4 xl:grid-cols-2">
			{#each organizationAccess as access (access.organization.id)}
				<div
					class="bg-card border-border flex h-full flex-col gap-4 rounded-2xl border p-6"
				>
					<div class="flex items-start justify-between gap-4">
						<div class="min-w-0">
							<h3 class="truncate text-xl font-semibold">
								{access.organization.name}
							</h3>
							{#if access.organization.description}
								<p class="text-muted-foreground mt-2 text-base">
									{access.organization.description}
								</p>
							{/if}
						</div>
					</div>

					<div class="mt-auto pt-2">
						<Button
							variant="outline"
							href={`/admin/organizations/${access.organization.id}`}
						>
							View details
						</Button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</section>
