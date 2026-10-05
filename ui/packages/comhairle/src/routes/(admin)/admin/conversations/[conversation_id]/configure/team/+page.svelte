<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<PageHeader title="Team" description="Manage collaborators." />

<RoleAssignments
	resourceType="conversation"
	resourceId={data.conversation.id}
	roleManagement={data.roleManagement}
/>
