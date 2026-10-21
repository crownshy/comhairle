<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import { SYSTEM_RESOURCE_ID } from '$lib/utils/permissions';
	import type { PageProps } from './$types';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';

	let { data }: PageProps = $props();
	let roleAssignments = $state<ReturnType<typeof RoleAssignments>>();
</script>

<SectionHeader
	content={{
		eyebrow: 'Administration',
		title: 'System roles',
		description: 'Choose who can manage the whole platform and what each person can do.'
	}}
>
	{#snippet actions()}
		{#if data.roleManagement.err === null}
			<Button onclick={() => roleAssignments?.addRecipient()}>
				<Plus class="size-4" /> Add people
			</Button>
		{/if}
	{/snippet}
</SectionHeader>
<TabContent>
	<RoleAssignments
		bind:this={roleAssignments}
		resourceType="system"
		resourceId={SYSTEM_RESOURCE_ID}
		roleManagement={data.roleManagement}
		standalone
	/>
</TabContent>
