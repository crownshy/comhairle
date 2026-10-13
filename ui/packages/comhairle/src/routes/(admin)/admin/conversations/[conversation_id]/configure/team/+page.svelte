<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import HostingOrganizationSection from './HostingOrganizationSection.svelte';
	import TransferOwnershipSection from './TransferOwnershipSection.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();

	const canTransferOwnership = $derived(data.isConversationOwner || data.isSuperAdmin);
</script>

<PageHeader
	title="Team"
	description="Manage the hosting organisation, ownership, collaborators and role assignments."
/>

<HostingOrganizationSection
	conversationId={data.conversation.id}
	organizationId={data.conversation.organizationId ?? null}
	organizations={data.userOrganizations.organizations}
/>

{#if canTransferOwnership}
	<TransferOwnershipSection conversationId={data.conversation.id} />
{/if}

<RoleAssignments
	resourceType="conversation"
	resourceId={data.conversation.id}
	roleManagement={data.roleManagement}
/>
