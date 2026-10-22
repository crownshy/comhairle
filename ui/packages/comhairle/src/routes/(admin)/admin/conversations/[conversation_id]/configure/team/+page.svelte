<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Button } from '$lib/components/ui/button';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import HostingOrganizationSection from './HostingOrganizationSection.svelte';
	import TransferOwnershipSection from './TransferOwnershipSection.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { teamConfigTour } from './tour';
	import type { PageProps } from './$types';

	useTour(() => teamConfigTour);

	let { data }: PageProps = $props();
	let roleAssignments = $state<ReturnType<typeof RoleAssignments>>();

	const canTransferOwnership = $derived(data.isConversationOwner || data.isSuperAdmin);
</script>

<PageHeader
	data-tour="page-header"
	level="h2"
	title="Team"
	description="Manage the hosting organisation, ownership, collaborators and role assignments."
>
	{#snippet actions()}
		{#if data.roleManagement?.err === null}
			<Button onclick={() => roleAssignments?.addRecipient()}>
				<Plus class="size-4" /> Add people
			</Button>
		{/if}
	{/snippet}
</PageHeader>

<HostingOrganizationSection
	conversationId={data.conversation.id}
	organizationId={data.conversation.organizationId ?? null}
	organizations={data.userOrganizations.organizations}
/>

{#if canTransferOwnership}
	<TransferOwnershipSection conversationId={data.conversation.id} />
{/if}

<RoleAssignments
	bind:this={roleAssignments}
	resourceType="conversation"
	resourceId={data.conversation.id}
	roleManagement={data.roleManagement}
	standalone
/>
