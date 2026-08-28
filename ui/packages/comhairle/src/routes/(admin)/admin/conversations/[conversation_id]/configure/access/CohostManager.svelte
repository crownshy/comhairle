<script lang="ts">
	import { onMount } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import type {
		LocalizedOrganizationDto,
		OrganizationWithPermissionDto
	} from '@crownshy/api-client/api';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import Combobox from '$lib/components/ui/combobox/combobox.svelte';
	import { Button } from '$lib/components/ui/button';
	import { LoaderCircle, Trash } from 'lucide-svelte';

	type Props = {
		conversationId: string;
		primaryHostOrganizationId: string | null;
		cohostOrganizations: OrganizationWithPermissionDto[];
		canManage: boolean;
	};

	let { conversationId, primaryHostOrganizationId, cohostOrganizations, canManage }: Props =
		$props();

	let adding = $state(false);
	let loadingOrganizations = $state(false);
	let removingOrganizationId = $state<string | null>(null);
	let selectedOrganization = $state<{ value: string; label: string } | undefined>(undefined);
	let organizations = $state<LocalizedOrganizationDto[]>([]);

	onMount(async () => {
		if (!canManage) return;

		loadingOrganizations = true;
		const response = await tryCatchAsync(() =>
			apiClient.ListOrganizations({
				queries: { limit: 500 }
			})
		);
		loadingOrganizations = false;

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Failed to load organizations',
				priority: 'ERROR'
			});
			return;
		}

		organizations = response.ok.records;
	});

	const cohostOrganizationIds = $derived(
		new Set(cohostOrganizations.map((organization) => organization.id))
	);
	const availableOptions = $derived(
		organizations
			.filter((organization) => organization.id !== primaryHostOrganizationId)
			.filter((organization) => !cohostOrganizationIds.has(organization.id))
			.map((organization) => ({ value: organization.id, label: organization.name }))
			.sort((left, right) => left.label.localeCompare(right.label))
	);

	async function addCohost() {
		if (!selectedOrganization) return;

		adding = true;
		const response = await tryCatchAsync(() =>
			apiClient.AddConversationCoHostOrganization(
				{ organization_id: selectedOrganization.value },
				{ params: { conversation_id: conversationId } }
			)
		);
		adding = false;

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Failed to add co-host organization',
				priority: 'ERROR'
			});
			return;
		}

		selectedOrganization = undefined;
		notifications.send({
			message: 'Co-host organization added',
			priority: 'INFO'
		});
		await invalidate('conversation:meta');
	}

	async function removeCohost(organizationId: string) {
		removingOrganizationId = organizationId;
		const response = await tryCatchAsync(() =>
			apiClient.RemoveConversationCoHostOrganization(undefined, {
				params: { conversation_id: conversationId, cohost_id: organizationId }
			})
		);
		removingOrganizationId = null;

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Failed to remove co-host organization',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: 'Co-host organization removed',
			priority: 'INFO'
		});
		await invalidate('conversation:meta');
	}
</script>

<section class="flex flex-col gap-4">
	<div class="flex flex-col gap-1">
		<p class="text-muted-foreground text-sm">
			Grant read access to additional organizations on this conversation.
		</p>
	</div>

	{#if canManage}
		<div class="flex flex-col gap-3 rounded-lg border p-4 md:flex-row md:items-center">
			<div class="min-w-0 flex-1">
				<Combobox
					items={availableOptions}
					selectedItem={selectedOrganization}
					placeholder="Search organizations by name"
					emptyMessage={loadingOrganizations
						? 'Loading organizations...'
						: 'No organizations available'}
					onSelect={(item) => {
						selectedOrganization = item;
					}}
				/>
			</div>
			<Button
				type="button"
				disabled={!selectedOrganization || adding || loadingOrganizations}
				onclick={addCohost}
			>
				{#if adding}
					<LoaderCircle class="mr-2 size-4 animate-spin" />
				{/if}
				Add co-host
			</Button>
		</div>
	{/if}

	<Card.Root>
		<Card.Content class="pt-6">
			{#if cohostOrganizations.length === 0}
				<p class="text-muted-foreground text-sm">No co-hosting organizations added yet.</p>
			{:else}
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head>Organization</Table.Head>
							<Table.Head>Role</Table.Head>
							{#if canManage}
								<Table.Head class="w-24">Actions</Table.Head>
							{/if}
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each cohostOrganizations as organization (organization.id)}
							<Table.Row>
								<Table.Cell>{organization.name}</Table.Cell>
								<Table.Cell>{organization.roleName}</Table.Cell>
								{#if canManage}
									<Table.Cell>
										<Button
											type="button"
											variant="ghost"
											size="icon"
											disabled={removingOrganizationId === organization.id}
											onclick={() => removeCohost(organization.id)}
										>
											{#if removingOrganizationId === organization.id}
												<LoaderCircle class="size-4 animate-spin" />
											{:else}
												<Trash class="size-4" />
											{/if}
											<span class="sr-only">Remove co-host</span>
										</Button>
									</Table.Cell>
								{/if}
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			{/if}
		</Card.Content>
	</Card.Root>
</section>
