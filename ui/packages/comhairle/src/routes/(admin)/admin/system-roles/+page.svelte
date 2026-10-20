<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { Button } from '$lib/components/ui/button';
	import RoleAssignments from '$lib/components/permissions/RoleAssignments.svelte';
	import { SYSTEM_RESOURCE_ID } from '$lib/utils/permissions';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { systemRolesTour } from './tour';
	import type { PageProps } from './$types';

	useTour(() => systemRolesTour);

	let { data }: PageProps = $props();
	let roleAssignments = $state<ReturnType<typeof RoleAssignments>>();
</script>

<section class="pt-page-top px-gutter max-w-300 grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	<PageHeader
		data-tour="page-header"
		title="System roles"
		description="Choose who can manage the whole platform and what each person can do."
	>
		{#snippet actions()}
			{#if data.roleManagement.err === null}
				<Button onclick={() => roleAssignments?.addRecipient()}>
					<Plus class="size-4" /> Add people
				</Button>
			{/if}
		{/snippet}
	</PageHeader>

	<RoleAssignments
		bind:this={roleAssignments}
		resourceType="system"
		resourceId={SYSTEM_RESOURCE_ID}
		roleManagement={data.roleManagement}
		standalone
	/>
</section>
