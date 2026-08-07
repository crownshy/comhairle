<script lang="ts">
	import type { PageProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Button } from '$lib/components/ui/button';
	import { Building2, Plus } from 'lucide-svelte';

	let props: PageProps = $props();
	let organizationAccess = $derived(props.data.userOrganizations?.organizations ?? []);
	let canCreateOrganization = $derived(
		props.data.userOrganizations?.canCreateOrganization ?? false
	);
</script>

<svelte:head>
	<title>Organisations - Comhairle Admin</title>
</svelte:head>

<div
	class="bg-muted flex w-full flex-col justify-between gap-11 border-b-black px-4 py-6 sm:px-8 md:px-16 md:py-8"
>
	<Breadcrumb.Root>
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/admin">Workspace</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>Organisations</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>

	<div class="mb-10 flex flex-col items-start gap-4 lg:flex-row lg:justify-between">
		<div class="flex items-center gap-2">
			<Building2 class="size-7 sm:size-9" />
			<h1 class="text-2xl sm:text-4xl">Your organisations</h1>
		</div>

		{#if canCreateOrganization}
			<Button class="w-full sm:w-auto" href="/admin/organizations/new">
				<Plus class="size-4" />
				Create New Organisation
			</Button>
		{/if}
	</div>

	<section class="flex w-full flex-col gap-6 overflow-y-auto">
		<h2 class="text-muted-foreground text-base font-medium">Your Organisations</h2>

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
</div>
