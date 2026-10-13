<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { page } from '$app/state';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ApiError, Snapshot } from '@crownshy/api-client/api';
	import { Building2, UserRound, Plus, Search, Pencil } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import * as Table from '$lib/components/ui/table';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { snakeToStartCase } from '$lib/utils/casingUtils';
	import { notifications } from '$lib/notifications.svelte';
	import {
		roleRecipients,
		type RoleRecipient,
		type RoleManagement,
		type RoleResourceType
	} from './roleAssignments';
	import { key } from '$lib/utils/invalidationKey';
	import RoleAssignmentDialog from './RoleAssignmentDialog.svelte';

	type Props = {
		resourceType: RoleResourceType;
		resourceId: string;
		roleManagement: RoleManagement | null;
	};
	let { resourceType, resourceId, roleManagement }: Props = $props();
	const canAdmin = $derived.by(() => {
		if (resourceType === 'system') return page.data.isSuperAdmin === true;
		if (resourceType === 'conversation')
			return permissions.can('conversation', 'conversation_admin', resourceId);
		return (
			permissions.can('organization', 'grant_permission', resourceId) &&
			permissions.can('organization', 'revoke_permission', resourceId)
		);
	});
	const permissionsKey = $derived.by(() => {
		if (resourceType === 'system') return key('admin/system/permissions');
		if (resourceType === 'conversation') return key('admin/conversation/permissions');
		return key('admin/organization/permissions');
	});
	const detailsKey = $derived.by(() => {
		if (resourceType === 'system') return key('user');
		if (resourceType === 'conversation') return key('admin/conversation/meta');
		return key('admin/organization/details');
	});
	const grantReason = $derived.by(() => {
		if (resourceType === 'system') return 'System access configuration';
		if (resourceType === 'conversation') return 'Conversation access configuration';
		return 'Organization access configuration';
	});
	const roleData = $derived(roleManagement?.err === null ? roleManagement.ok : null);
	const recipients = $derived(
		roleData ? roleRecipients(roleData.assignments, roleData.users, roleData.organizations) : []
	);
	let search = $state('');
	let kindFilter = $state('all');
	let roleFilter = $state('all');
	const filteredRecipients = $derived(
		recipients.filter(
			(recipient) =>
				(kindFilter === 'all' || recipient.kind === kindFilter) &&
				(roleFilter === 'all' || recipient.roles.includes(roleFilter)) &&
				`${recipient.name} ${recipient.detail}`.toLowerCase().includes(search.toLowerCase())
		)
	);

	let open = $state(false);
	let recipient = $state<RoleRecipient | null>(null);
	let snapshot = $state<Snapshot | null>(null);
	let selectedRoles = $state<string[]>([]);
	let loading = $state(false);
	let saving = $state(false);
	let error = $state('');
	let newKind = $state<'user' | 'organization'>('user');
	let email = $state('');
	let organizationId = $state('');
	const params = $derived({
		resource_type: resourceType,
		resource_id: resourceId
	});
	const hasChanges = $derived(
		!snapshot ||
			selectedRoles.length !== snapshot.roles.length ||
			selectedRoles.some((role) => !snapshot?.roles.includes(role))
	);

	function addRecipient() {
		if (!canAdmin || loading || saving) return;
		recipient = null;
		snapshot = null;
		selectedRoles = [];
		email = '';
		organizationId = '';
		newKind = 'user';
		error = '';
		open = true;
	}

	function loadRecipientAssignments(target: RoleRecipient) {
		return apiClient.GetPermissionAssignments({
			params: { ...params, recipient_type: target.type, recipient_id: target.id }
		});
	}

	async function editRecipient(target: RoleRecipient) {
		if (!canAdmin || loading || saving) return;
		recipient = target;
		snapshot = null;
		selectedRoles = [];
		error = '';
		open = true;
		loading = true;
		const result = await tryCatchAsync(() => loadRecipientAssignments(target));
		loading = false;
		if (result.err !== null) {
			error = 'Could not load role assignments.';
			return;
		}
		snapshot = result.ok;
		selectedRoles = result.ok.roles;
	}

	function toggleRole(role: string, checked: boolean) {
		if (!canAdmin || loading || saving) return;
		const index = selectedRoles.findIndex((selectedRole) => selectedRole === role);
		if (checked) {
			// If already exists then no-op
			if (index > -1) return;

			selectedRoles.push(role);
			return;
		}
		// If already doesn't exist then no-op
		if (index < 0) return;

		selectedRoles.splice(index, 1);
	}

	async function resolveUserRecipient(): Promise<RoleRecipient> {
		const existing = roleData?.users.find(
			(user) => user.email?.toLowerCase() === email.trim().toLowerCase()
		);
		let userId = existing?.id;
		if (!userId) {
			const grant = await apiClient.GrantPermission(
				{
					user_email: email.trim(),
					role_name: selectedRoles[0],
					grant_reason: grantReason
				},
				{ params }
			);
			userId = grant.user_id ?? undefined;
		}
		if (!userId) throw new Error('Could not resolve the user');
		return {
			id: userId,
			type: 'user',
			kind: 'User',
			name: email.trim(),
			detail: email.trim(),
			roles: []
		};
	}

	function resolveOrganizationRecipient(): RoleRecipient {
		const organization = roleData?.organizations.find(
			(organization) => organization.id === organizationId
		);
		if (!organization) throw new Error('Select an organization');
		return {
			id: organization.userGroupId,
			type: 'group',
			kind: 'Organization',
			name: organization.name,
			detail: organization.contactEmail ?? organization.id,
			roles: []
		};
	}

	async function prepareNewRecipient(): Promise<RoleRecipient> {
		const target =
			newKind === 'user' ? await resolveUserRecipient() : resolveOrganizationRecipient();
		recipient = target;
		const current = await loadRecipientAssignments(target);
		snapshot = current;
		selectedRoles = Array.from(new Set(current.roles.concat(selectedRoles)));
		return target;
	}

	async function persistRoleAssignments(): Promise<Snapshot> {
		const target = recipient ?? (await prepareNewRecipient());
		const current = snapshot;
		if (!current || !canAdmin) throw new Error('Cannot save assignments');
		return apiClient.SavePermissionAssignments(
			{
				roles: selectedRoles,
				expected_version: current.version,
				grant_reason: grantReason
			},
			{ params: { ...params, recipient_type: target.type, recipient_id: target.id } }
		);
	}

	async function saveRoles() {
		if (!canAdmin || loading || saving || !hasChanges) return;
		if (!recipient && selectedRoles.length === 0) return;
		saving = true;
		error = '';
		const result = await tryCatchAsync<Snapshot, ApiError>(persistRoleAssignments);
		saving = false;
		if (result.err !== null) {
			error =
				result.err.response?.status === 409
					? 'Roles changed since this dialog opened. Reload assignments before saving.'
					: 'Could not save roles. Check the recipient and try again.';
			await invalidate(permissionsKey);
			return;
		}
		snapshot = result.ok;
		open = false;
		notifications.send({ message: 'Roles updated', priority: 'INFO' });
		await Promise.all([invalidate(permissionsKey), invalidate(detailsKey)]);
	}
</script>

<section class="border-border border-t py-6">
	<h3 class="mb-4 text-base font-semibold">Role assignments</h3>
	{#if !canAdmin}
		<p class="text-muted-foreground text-base">
			You do not have permission to manage role assignments.
		</p>
	{:else if !roleData}
		<div class="text-destructive text-base" role="alert">
			{#if roleManagement?.err}
				{#each roleManagement.err as loadError (loadError.source)}
					<p>{loadError.message}</p>
				{/each}
			{:else}
				<p>Could not load role assignments.</p>
			{/if}
		</div>
		<Button variant="outline" onclick={() => invalidate(permissionsKey)}>Retry</Button>
	{:else}
		<div class="mb-4 flex flex-wrap items-center gap-3">
			<div class="relative min-w-0 flex-1">
				<Search class="text-muted-foreground absolute top-3 left-3 size-4" />
				<Input
					bind:value={search}
					placeholder="Search users, emails or organizations"
					aria-label="Search role assignments"
					class="pl-9"
				/>
			</div>
			<select
				bind:value={kindFilter}
				aria-label="Recipient type"
				class="border-input bg-background h-10 rounded-md border px-3 text-base"
			>
				<option value="all">All types</option>
				<option>User</option>
				<option>Organization</option>
			</select>
			<select
				bind:value={roleFilter}
				aria-label="Filter by role"
				class="border-input bg-background h-10 rounded-md border px-3 text-base"
			>
				<option value="all">All roles</option>
				{#each roleData.roles as role (role)}<option value={role}
						>{snakeToStartCase(role)}</option
					>{/each}
			</select>
			<Button onclick={addRecipient} disabled={!canAdmin || loading || saving}>
				<Plus class="size-4" /> Add recipient</Button
			>
		</div>
		<div class="border-border overflow-x-auto rounded-lg border">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Recipient</Table.Head><Table.Head>Email / ID</Table.Head>
						<Table.Head>Type</Table.Head><Table.Head>Current roles</Table.Head>
						<Table.Head class="text-right">Actions</Table.Head></Table.Row
					>
				</Table.Header>
				<Table.Body>
					{#each filteredRecipients as target (`${target.type}:${target.id}`)}
						<Table.Row>
							<Table.Cell>
								<div class="flex items-center gap-2">
									{#if target.kind === 'Organization'}<Building2
											class="size-5 shrink-0"
										/>{:else}<UserRound class="size-5 shrink-0" />{/if}<span
										class="max-w-60 truncate text-base font-medium"
										title={target.name}>{target.name}</span
									>
								</div></Table.Cell
							>
							<Table.Cell class="max-w-60 truncate text-base" title={target.detail}
								>{target.detail}</Table.Cell
							>
							<Table.Cell class="text-base">{target.kind}</Table.Cell>
							<Table.Cell>
								<div class="flex flex-wrap gap-1">
									{#each target.roles as role (role)}<Badge
											variant="secondary"
											class="text-base">{snakeToStartCase(role)}</Badge
										>{/each}
								</div></Table.Cell
							>
							<Table.Cell class="text-right">
								<Button
									variant="outline"
									onclick={() => editRecipient(target)}
									disabled={!canAdmin || loading || saving}
								>
									<Pencil class="size-4" /> Edit roles</Button
								>
							</Table.Cell>
						</Table.Row>
					{:else}<Table.Row>
							<Table.Cell
								colspan={5}
								class="text-muted-foreground py-8 text-center text-base"
								>{recipients.length
									? 'No matching recipients.'
									: 'No assigned roles.'}</Table.Cell
							>
						</Table.Row>{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/if}
</section>

<RoleAssignmentDialog
	bind:open
	bind:newKind
	bind:email
	bind:organizationId
	{recipient}
	{snapshot}
	{selectedRoles}
	{loading}
	{saving}
	{error}
	{canAdmin}
	{roleData}
	{hasChanges}
	onToggleRole={toggleRole}
	onSave={saveRoles}
	onReload={editRecipient}
/>
