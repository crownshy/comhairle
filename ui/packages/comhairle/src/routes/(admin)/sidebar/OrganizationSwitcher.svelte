<script lang="ts">
	import { resolve } from '$app/paths';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Building2, ChevronDown } from 'lucide-svelte';
	import type { UserOrganizationAccess } from '@crownshy/api-client/api';
	import { userInitials } from '$lib/utils';

	// Only navigates for now. Scoping the lists to one organisation is a later step.
	let { organizations }: { organizations: UserOrganizationAccess[] } = $props();

	let first = $derived(organizations[0]?.organization);
</script>

{#if first}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger
			class="hover:bg-sidebar-accent flex min-w-0 flex-1 items-center gap-2 rounded-lg p-1 text-left"
		>
			<span
				class="bg-primary text-primary-foreground flex size-8 shrink-0 items-center justify-center rounded-lg text-sm font-semibold"
				aria-hidden="true"
			>
				{userInitials(first.name)}
			</span>
			<span class="text-sidebar-foreground min-w-0 flex-1 truncate text-base font-semibold">
				{first.name}
			</span>
			<ChevronDown class="text-sidebar-foreground/70 size-4 shrink-0" />
		</DropdownMenu.Trigger>
		<DropdownMenu.Content align="start" class="w-64">
			{#each organizations as access (access.organization.id)}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a
							{...props}
							href={resolve('/(admin)/admin/organizations/[organization_id]', {
								organization_id: access.organization.id
							})}
						>
							<Building2 class="size-4" />
							<span class="truncate">{access.organization.name}</span>
						</a>
					{/snippet}
				</DropdownMenu.Item>
			{/each}
			<DropdownMenu.Separator />
			<DropdownMenu.Item>
				{#snippet child({ props })}
					<a {...props} href={resolve('/admin/organizations')}>All organisations</a>
				{/snippet}
			</DropdownMenu.Item>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/if}
