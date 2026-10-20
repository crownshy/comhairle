<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Button } from '$lib/components/ui/button';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { teamConfigTour } from './tour';
	import type { PageProps } from './$types';

	useTour(() => teamConfigTour);

	let { data }: PageProps = $props();
	let roleAssignments = $state<ReturnType<typeof RoleAssignments>>();
</script>

<PageHeader
	data-tour="page-header"
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
