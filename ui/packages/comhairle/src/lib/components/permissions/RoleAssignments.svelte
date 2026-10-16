<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { page } from '$app/state';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ApiError, EmailLookup, Snapshot } from '@crownshy/api-client/api';
	import { Building2, UserRound, Plus, Search, Pencil } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { cn } from '$lib/utils';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { snakeToStartCase } from '$lib/utils/casingUtils';
	import { notifications } from '$lib/notifications.svelte';
	import {
		roleCounts,
		roleDescription,
		roleRecipients,
		parseEmails,
		type RoleRecipient,
		type RoleManagement,
		type RoleResourceType
	} from './roleAssignments';
	import { key } from '$lib/utils/invalidationKey';
	import RoleAssignmentDialog from './RoleAssignmentDialog.svelte';
	import InviteUnregisteredDialog from './InviteUnregisteredDialog.svelte';

	type Props = {
		resourceType: RoleResourceType;
		resourceId: string;
		roleManagement: RoleManagement | null;
		/**
		 * The list is the whole page: the page header carries the title and calls
		 * `addRecipient` from its own Add button, so this component renders neither.
		 */
		standalone?: boolean;
	};
	let { resourceType, resourceId, roleManagement, standalone = false }: Props = $props();
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
	let kindFilter = $state<'all' | RoleRecipient['kind']>('all');
	let roleFilter = $state('all');
	const filteredRecipients = $derived(
		recipients.filter(
			(recipient) =>
				(kindFilter === 'all' || recipient.kind === kindFilter) &&
				(roleFilter === 'all' || recipient.roles.includes(roleFilter)) &&
				`${recipient.name} ${recipient.detail}`.toLowerCase().includes(search.toLowerCase())
		)
	);
	const rolesWithCounts = $derived(roleData ? roleCounts(roleData.roles, recipients) : []);
	const hasOrganizationRecipients = $derived(
		recipients.some((recipient) => recipient.kind === 'Organization')
	);
	const isFiltered = $derived(search !== '' || kindFilter !== 'all' || roleFilter !== 'all');
	function countLabel(count: number, singular: string, plural: string) {
		return `${count} ${count === 1 ? singular : plural}`;
	}
	const resultSummary = $derived.by(() => {
		if (isFiltered) return `Showing ${filteredRecipients.length} of ${recipients.length}`;
		const people = recipients.filter((recipient) => recipient.kind === 'User').length;
		const organizations = recipients.length - people;
		const parts = [countLabel(people, 'person', 'people')];
		if (organizations > 0)
			parts.push(countLabel(organizations, 'organization', 'organizations'));
		return `${parts.join(' and ')} with roles`;
	});
	const kindOptions = [
		{ value: 'all', label: 'All' },
		{ value: 'User', label: 'People' },
		{ value: 'Organization', label: 'Organizations' }
	] as const;

	/** Second line under a name: the email or ID when it adds something, and the kind for organizations. */
	function recipientSubtitle(target: RoleRecipient) {
		const parts: string[] = [];
		if (target.kind === 'Organization') parts.push('Organization');
		if (target.detail !== target.name) parts.push(target.detail);
		return parts.join(' · ');
	}

	function clearFilters() {
		search = '';
		kindFilter = 'all';
		roleFilter = 'all';
	}

	function toggleRoleFilter(role: string) {
		roleFilter = roleFilter === role ? 'all' : role;
	}

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
	let inviteLookup = $state<EmailLookup | null>(null);
	let inviteDialogOpen = $state(false);
	const params = $derived({
		resource_type: resourceType,
		resource_id: resourceId
	});
	const hasChanges = $derived(
		!snapshot ||
			selectedRoles.length !== snapshot.roles.length ||
			selectedRoles.some((role) => !snapshot?.roles.includes(role))
	);

	export function addRecipient() {
		if (!canAdmin || !roleData || loading || saving) return;
		recipient = null;
		snapshot = null;
		selectedRoles = [];
		email = '';
		organizationId = '';
		newKind = 'user';
		error = '';
		inviteLookup = null;
		inviteDialogOpen = false;
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
		const target = resolveOrganizationRecipient();
		recipient = target;
		const current = await loadRecipientAssignments(target);
		snapshot = current;
		selectedRoles = Array.from(new Set(current.roles.concat(selectedRoles)));
		return target;
	}

	function saveRecipientRoles(target: RoleRecipient, current: Snapshot, roles: string[]) {
		return apiClient.SavePermissionAssignments(
			{
				roles,
				expected_version: current.version,
				grant_reason: grantReason
			},
			{ params: { ...params, recipient_type: target.type, recipient_id: target.id } }
		);
	}

	async function persistRoleAssignments(): Promise<Snapshot> {
		const target = recipient ?? (await prepareNewRecipient());
		const current = snapshot;
		if (!current || !canAdmin) throw new Error('Cannot save assignments');
		return saveRecipientRoles(target, current, selectedRoles);
	}

	async function assignRolesToEmails(lookup: EmailLookup) {
		for (const registered of lookup.registered) {
			const target: RoleRecipient = {
				id: registered.user_id,
				type: 'user',
				kind: 'User',
				name: registered.email,
				detail: registered.email,
				roles: []
			};
			const current = await loadRecipientAssignments(target);
			await saveRecipientRoles(
				target,
				current,
				Array.from(new Set(current.roles.concat(selectedRoles)))
			);
		}
		if (lookup.unregistered.length === 0) return;
		const invited = await apiClient.InviteToAssignRoles(
			{ emails: lookup.unregistered, roles: selectedRoles, grant_reason: grantReason },
			{ params }
		);
		if (invited.emails.length !== lookup.unregistered.length) {
			throw new Error(
				'Some recipients registered while the invitation was being prepared. Retry.'
			);
		}
	}

	async function runSave(save: () => Promise<Snapshot | void>, message: string) {
		saving = true;
		error = '';
		const result = await tryCatchAsync<Snapshot | void, ApiError>(save);
		saving = false;
		inviteDialogOpen = false;
		if (result.err !== null) {
			error =
				result.err.response?.status === 409
					? 'Roles changed since this dialog opened. Reload assignments before saving.'
					: 'Could not save roles. Check the recipient and try again.';
			await invalidate(permissionsKey);
			return;
		}
		if (result.ok) snapshot = result.ok;
		open = false;
		const contactEmail = result.ok?.invitation_sent_to;
		notifications.send({
			message: contactEmail
				? `Invitation sent to ${contactEmail}. The roles apply once the organization accepts`
				: message,
			priority: 'INFO'
		});
		await Promise.all([invalidate(permissionsKey), invalidate(detailsKey)]);
	}

	async function saveRoles() {
		if (!canAdmin || loading || saving || !hasChanges) return;
		if (!recipient && selectedRoles.length === 0) return;
		if (recipient || newKind === 'organization') {
			await runSave(persistRoleAssignments, 'Roles updated');
			return;
		}
		saving = true;
		error = '';
		const lookup = await tryCatchAsync(() =>
			apiClient.LookupAssignmentEmails({ emails: parseEmails(email) }, { params })
		);
		saving = false;
		if (lookup.err !== null) {
			error = 'Could not check the email addresses. Try again.';
			return;
		}
		if (lookup.ok.unregistered.length > 0) {
			inviteLookup = lookup.ok;
			inviteDialogOpen = true;
			return;
		}
		await runSave(() => assignRolesToEmails(lookup.ok), 'Roles updated');
	}

	async function confirmInvites() {
		const lookup = inviteLookup;
		if (!lookup || !canAdmin || saving) return;
		const count = lookup.unregistered.length;
		await runSave(
			() => assignRolesToEmails(lookup),
			`Roles updated. ${count} ${count === 1 ? 'invitation' : 'invitations'} sent`
		);
	}
</script>

{#snippet filterChip(
	label: string,
	count: number,
	isActive: boolean,
	onclick: () => void,
	title?: string
)}
	<button
		type="button"
		aria-pressed={isActive}
		{title}
		{onclick}
		class={cn(
			'border-border inline-flex items-center gap-2 rounded-full border px-3 py-1 text-base transition-colors',
			isActive
				? 'bg-primary text-primary-foreground border-primary'
				: 'bg-background hover:bg-muted'
		)}
	>
		{label}
		<span class={cn('text-sm tabular-nums', !isActive && 'text-muted-foreground')}>{count}</span
		>
	</button>
{/snippet}

{#snippet addButton()}
	<Button onclick={addRecipient} disabled={loading || saving}>
		<Plus class="size-4" /> Add people
	</Button>
{/snippet}

<section class={cn(!standalone && 'border-border border-t py-6')}>
	{#if !standalone}
		<div class="mb-4 flex items-center justify-between gap-4">
			<h3 class="text-base font-semibold">Role assignments</h3>
			{#if canAdmin && roleData && recipients.length > 0}{@render addButton()}{/if}
		</div>
	{/if}
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
	{:else if recipients.length === 0}
		<div
			class="border-border flex flex-col items-center gap-4 rounded-lg border border-dashed px-6 py-12 text-center"
		>
			<p class="text-base font-medium">No one has a role yet</p>
			<p class="text-muted-foreground max-w-md text-base">
				Add people or organizations and choose what each of them can do.
			</p>
			{@render addButton()}
		</div>
	{:else}
		<div class="flex flex-col gap-6">
			<div
				role="search"
				aria-label="Filter role assignments"
				class="bg-muted/50 border-border flex flex-col gap-4 rounded-lg border p-4"
			>
				<div class="flex flex-col gap-3 sm:flex-row sm:items-center">
					<div class="relative min-w-0 flex-1">
						<Search class="text-muted-foreground absolute top-3 left-3 size-4" />
						<Input
							bind:value={search}
							placeholder="Search by name, email or organization"
							aria-label="Search by name, email or organization"
							class="bg-background pl-9"
						/>
					</div>
					{#if hasOrganizationRecipients}
						<div
							role="group"
							aria-label="Recipient type"
							class="bg-muted inline-flex shrink-0 self-start rounded-md p-1 sm:self-auto"
						>
							{#each kindOptions as option (option.value)}
								<button
									type="button"
									aria-pressed={kindFilter === option.value}
									onclick={() => (kindFilter = option.value)}
									class={cn(
										'rounded px-3 py-1 text-base transition-colors',
										kindFilter === option.value
											? 'bg-background font-medium shadow-xs'
											: 'text-muted-foreground hover:text-foreground'
									)}
								>
									{option.label}
								</button>
							{/each}
						</div>
					{/if}
				</div>

				<div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:gap-3">
					<span
						id="role-filter-label"
						class="text-muted-foreground shrink-0 text-sm font-medium">Role</span
					>
					<div
						role="group"
						aria-labelledby="role-filter-label"
						class="flex flex-wrap gap-2"
					>
						{@render filterChip(
							'All',
							recipients.length,
							roleFilter === 'all',
							() => (roleFilter = 'all')
						)}
						{#each rolesWithCounts as { role, count } (role)}
							{@render filterChip(
								snakeToStartCase(role),
								count,
								roleFilter === role,
								() => toggleRoleFilter(role),
								roleDescription(resourceType, role)
							)}
						{/each}
					</div>
				</div>
			</div>

			<div class="flex items-center justify-between gap-4">
				<p class="text-muted-foreground text-base" aria-live="polite">{resultSummary}</p>
				{#if isFiltered}
					<Button variant="ghost" onclick={clearFilters}>Clear filters</Button>
				{/if}
			</div>

			{#if filteredRecipients.length === 0}
				<div
					class="border-border flex flex-col items-center gap-3 rounded-lg border px-6 py-10 text-center"
				>
					<p class="text-muted-foreground text-base">No one matches these filters.</p>
					<Button variant="outline" onclick={clearFilters}>Clear filters</Button>
				</div>
			{:else}
				<div class="border-border overflow-hidden rounded-lg border">
					<div
						aria-hidden="true"
						class="bg-muted/50 border-border text-muted-foreground hidden gap-6 border-b px-4 py-2 text-sm font-medium sm:flex"
					>
						<span class="flex-1">Name</span>
						<span class="w-80">Roles</span>
						<span class="w-24"></span>
					</div>
					<ul class="divide-border divide-y">
						{#each filteredRecipients as target (`${target.type}:${target.id}`)}
							<li
								class="flex flex-col gap-3 p-4 sm:flex-row sm:items-center sm:gap-6"
							>
								<div class="flex min-w-0 flex-1 items-center gap-3">
									<span
										class="bg-muted text-muted-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
										aria-hidden="true"
									>
										{#if target.kind === 'Organization'}
											<Building2 class="size-5" />
										{:else}
											<UserRound class="size-5" />
										{/if}
									</span>
									<div class="min-w-0">
										<p
											class="truncate text-base font-medium"
											title={target.name}
										>
											{target.name}
										</p>
										{#if recipientSubtitle(target)}
											<p
												class="text-muted-foreground truncate text-base"
												title={recipientSubtitle(target)}
											>
												{recipientSubtitle(target)}
											</p>
										{/if}
									</div>
								</div>
								<div class="flex flex-wrap gap-1 sm:w-80">
									{#each target.roles as role (role)}
										<Badge
											variant={role === roleFilter ? 'primary' : 'secondary'}
											class="text-base"
											title={roleDescription(resourceType, role)}
										>
											{snakeToStartCase(role)}
										</Badge>
									{/each}
								</div>
								<Button
									variant="ghost"
									class="self-start sm:w-24 sm:self-center"
									onclick={() => editRecipient(target)}
									disabled={loading || saving}
									aria-label={`Edit roles for ${target.name}`}
								>
									<Pencil class="size-4" /> Edit
								</Button>
							</li>
						{/each}
					</ul>
				</div>
			{/if}
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
	{resourceType}
	{hasChanges}
	onToggleRole={toggleRole}
	onSave={saveRoles}
	onReload={editRecipient}
/>

<InviteUnregisteredDialog
	bind:open={inviteDialogOpen}
	emails={inviteLookup?.unregistered ?? []}
	{saving}
	onConfirm={confirmInvites}
/>
