<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import { SYSTEM_RESOURCE_ID } from '$lib/utils/permissions';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<PageHeader
	title="System roles"
	description="Manage platform-wide role assignments for users and organizations."
/>

<div class="px-6">
	<RoleAssignments
		resourceType="system"
		resourceId={SYSTEM_RESOURCE_ID}
		roleManagement={data.roleManagement}
	/>
</div>
