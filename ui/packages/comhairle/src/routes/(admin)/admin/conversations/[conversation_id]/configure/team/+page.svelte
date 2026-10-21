<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Button } from '$lib/components/ui/button';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let roleAssignments = $state<ReturnType<typeof RoleAssignments>>();
</script>

<PageHeader
	level="h2"
	title="Team"
	description="Choose who works on this conversation and what each person can do."
>
	{#snippet actions()}
		{#if data.roleManagement?.err === null}
			<Button onclick={() => roleAssignments?.addRecipient()}>
				<Plus class="size-4" /> Add people
			</Button>
		{/if}
	{/snippet}
</PageHeader>

<RoleAssignments
	bind:this={roleAssignments}
	resourceType="conversation"
	resourceId={data.conversation.id}
	roleManagement={data.roleManagement}
	standalone
/>
