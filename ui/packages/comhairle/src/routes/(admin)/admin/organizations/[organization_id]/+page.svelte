<script lang="ts">
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { goto, invalidate } from '$app/navigation';
	import { page } from '$app/state';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { Trash2, SquarePen, Trash } from 'lucide-svelte';
	import type { PageData } from './$types';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	let { data }: { data: PageData } = $props();

	type TeamRole = 'member' | 'admin';
	type TeamMember = {
		id: string;
		username?: string | null;
		email?: string | null;
		role: TeamRole;
	};

	let isEditing = $state(false);
	let isDeleting = $state(false);
	let isSaving = $state(false);
	let teamBusy = $state(false);
	let confirmCreateUserOpen = $state(false);
	let pendingCreateUser = $state<{ email: string; role: TeamRole } | null>(null);

	let members = $state<TeamMember[]>(data.team.members ?? []);
	let memberEmail = $state('');
	let newMemberRole = $state<TeamRole>('member');

	let editName = $state(data.organization?.name ?? '');
	let editDescription = $state(data.organization?.description ?? '');
	let editMission = $state(data.organization?.mission ?? '');
	let editOrgType = $state<'non_profit' | 'governmental' | 'other'>(
		(data.organization?.orgType as 'non_profit' | 'governmental' | 'other' | undefined) ??
			'other'
	);
	let editContactEmail = $state(data.organization?.contactEmail ?? '');
	let editExternalUrl = $state(data.organization?.externalUrl ?? '');
	let editRegionIds = $state<string[]>(data.organization?.regions ?? []);

	let activeTab = $derived(page.url.searchParams.get('tab') ?? 'details');
	let currentUserId = $derived(data.user?.id ?? '');

	$effect(() => {
		members = data.team.members ?? [];
	});

	async function setTab(tab: 'details' | 'team') {
		const params = new URLSearchParams(page.url.searchParams);
		params.set('tab', tab);
		await goto(`?${params.toString()}`, {
			replaceState: true,
			noScroll: true,
			keepFocus: true
		});
	}

	function startEditing() {
		editName = data.organization?.name ?? '';
		editDescription = data.organization?.description ?? '';
		editMission = data.organization?.mission ?? '';
		editOrgType =
			(data.organization?.orgType as 'non_profit' | 'governmental' | 'other' | undefined) ??
			'other';
		editContactEmail = data.organization?.contactEmail ?? '';
		editExternalUrl = data.organization?.externalUrl ?? '';
		editRegionIds = [...(data.organization?.regions ?? [])];
		isEditing = true;
	}

	function toggleRegion(regionId: string, enabled: boolean) {
		if (enabled) {
			if (!editRegionIds.includes(regionId)) {
				editRegionIds = [...editRegionIds, regionId];
			}
			return;
		}

		editRegionIds = editRegionIds.filter((id) => id !== regionId);
	}

	function cancelEditing() {
		isEditing = false;
	}

	async function saveOrganization() {
		if (!data.organization) return;

		isSaving = true;
		try {
			await apiClient.UpdateOrganization(
				{
					name: editName || undefined,
					description: editDescription || undefined,
					mission: editMission || undefined,
					org_type: editOrgType,
					contact_email: editContactEmail || null,
					external_url: editExternalUrl || null,
					regions: editRegionIds
				},
				{ params: { organization_id: data.organization.id } }
			);

			notifications.send({ priority: 'INFO', message: 'Organization updated' });
			await invalidate('organization:details');
			isEditing = false;
		} catch (error) {
			console.error(error);
			notifications.send({ priority: 'ERROR', message: 'Failed to update organization' });
		} finally {
			isSaving = false;
		}
	}

	async function refreshTeam() {
		await invalidate('organization:team');
	}

	async function addMember(allowCreateUser = false, preset?: { email: string; role: TeamRole }) {
		const email = (preset?.email ?? memberEmail).trim();
		const requestedRole = preset?.role ?? newMemberRole;
		if (email.length === 0 || !data.organization) return;

		teamBusy = true;
		const response = await tryCatchAsync(async () => {
			const result = await fetch(`/api/organizations/${data.organization.id}/members`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					email,
					role: requestedRole,
					allow_create_user: allowCreateUser
				})
			});

			if (result.status === 404 && !allowCreateUser) {
				return { requiresConfirmation: true as const };
			}

			if (!result.ok) {
				throw new Error(`Failed to add member (${result.status})`);
			}
			return result.json() as Promise<{ createdAccount: boolean; emailed: boolean }>;
		});

		if (response.ok !== null && 'requiresConfirmation' in response.ok) {
			pendingCreateUser = { email, role: requestedRole };
			confirmCreateUserOpen = true;
			teamBusy = false;
			return;
		}

		if (response.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to add organization member' });
			teamBusy = false;
			return;
		}

		memberEmail = '';
		newMemberRole = 'member';
		pendingCreateUser = null;
		confirmCreateUserOpen = false;
		await refreshTeam();

		notifications.send({
			priority: response.ok.createdAccount && !response.ok.emailed ? 'WARNING' : 'INFO',
			message: response.ok.createdAccount
				? response.ok.emailed
					? 'Member added and account created. Account setup email sent.'
					: 'Member added and account created. Account setup email could not be sent.'
				: 'Member added to organization'
		});

		teamBusy = false;
	}

	async function confirmCreateUser() {
		if (!pendingCreateUser) return;
		await addMember(true, pendingCreateUser);
	}

	async function removeMember(userId: string) {
		if (!data.organization) return;

		teamBusy = true;
		const response = await tryCatchAsync(async () => {
			const result = await fetch(
				`/api/organizations/${data.organization.id}/members/${userId}`,
				{
					method: 'DELETE'
				}
			);
			if (!result.ok) {
				throw new Error(`Failed to remove member (${result.status})`);
			}
		});

		if (response.err !== null) {
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to remove organization member'
			});
			teamBusy = false;
			return;
		}

		await refreshTeam();
		notifications.send({ priority: 'INFO', message: 'Member removed from organization' });
		teamBusy = false;
	}

	async function updateMemberRole(userId: string, role: TeamRole) {
		if (!data.organization) return;

		teamBusy = true;
		const response = await tryCatchAsync(async () => {
			const result = await fetch(
				`/api/organizations/${data.organization.id}/members/${userId}/role`,
				{
					method: 'PUT',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ role })
				}
			);
			if (!result.ok) {
				throw new Error(`Failed to update role (${result.status})`);
			}
		});

		if (response.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to update member role' });
			teamBusy = false;
			return;
		}

		await refreshTeam();
		notifications.send({ priority: 'INFO', message: 'Member role updated' });
		teamBusy = false;
	}

	async function deleteOrganization() {
		if (!data.organization) return;

		try {
			await apiClient.DeleteOrganization(undefined, {
				params: { organization_id: data.organization.id }
			});

			await Promise.all([
				invalidate('organization:details'),
				invalidate('admin:organizations')
			]);
			notifications.send({ priority: 'INFO', message: 'Organization deleted' });
			goto('/admin');
		} catch (error) {
			console.error(error);
			notifications.send({ priority: 'ERROR', message: 'Failed to delete organization' });
		} finally {
			isDeleting = false;
		}
	}
</script>

<svelte:head>
	<title>{data.organization?.name ?? 'Organization'} - Comhairle Admin</title>
</svelte:head>

<div class="flex flex-col gap-6 p-8">
	{#if data.organization}
		<div class="flex flex-col gap-2">
			<div class="flex items-center justify-between">
				<div class="flex-1">
					<h1 class="text-3xl font-semibold">{data.organization.name}</h1>
					{#if data.organization.description}
						<p class="text-muted-foreground">{data.organization.description}</p>
					{/if}
				</div>

				{#if data.canEdit || data.canDelete}
					<div class="flex gap-2">
						{#if data.canEdit}
							<Button
								variant="outline"
								size="icon"
								onclick={startEditing}
								title="Edit organization"
							>
								<SquarePen class="h-4 w-4" />
								<span class="sr-only">Edit organization</span>
							</Button>
						{/if}
						{#if data.canDelete}
							<Button
								variant="destructive"
								size="icon"
								onclick={() => (isDeleting = true)}
								title="Delete organization"
							>
								<Trash2 class="h-4 w-4" />
								<span class="sr-only">Delete organization</span>
							</Button>
						{/if}
					</div>
				{/if}
			</div>
		</div>

		<div class="border-b">
			<div class="flex gap-4">
				<button
					type="button"
					onclick={() => setTab('details')}
					class={`border-b-2 px-3 py-2 text-sm font-medium ${
						activeTab === 'details'
							? 'border-foreground text-foreground'
							: 'text-muted-foreground hover:text-foreground border-transparent'
					}`}
				>
					Details
				</button>
				<button
					type="button"
					onclick={() => setTab('team')}
					class={`border-b-2 px-3 py-2 text-sm font-medium ${
						activeTab === 'team'
							? 'border-foreground text-foreground'
							: 'text-muted-foreground hover:text-foreground border-transparent'
					}`}
				>
					Team
				</button>
			</div>
		</div>

		{#if activeTab === 'details'}
			{#if isEditing}
				<div class="rounded-lg border p-4">
					<h2 class="mb-4 text-lg font-semibold">Edit Organization</h2>
					<div class="flex flex-col gap-4">
						<div class="flex flex-col gap-2">
							<Label for="edit-name">Name</Label>
							<Input
								id="edit-name"
								bind:value={editName}
								placeholder="Organization name"
								required
							/>
						</div>

						<div class="flex flex-col gap-2">
							<Label for="edit-description">Description</Label>
							<textarea
								id="edit-description"
								bind:value={editDescription}
								class="border-input bg-background min-h-24 rounded-md border px-3 py-2 text-sm"
								placeholder="Organization description"
							></textarea>
						</div>

						<div class="flex flex-col gap-2">
							<Label for="edit-mission">Mission</Label>
							<textarea
								id="edit-mission"
								bind:value={editMission}
								class="border-input bg-background min-h-24 rounded-md border px-3 py-2 text-sm"
								placeholder="Organization mission"
							></textarea>
						</div>

						<div class="flex flex-col gap-2">
							<Label for="edit-org-type">Type</Label>
							<select
								id="edit-org-type"
								bind:value={editOrgType}
								class="border-input bg-background h-10 rounded-md border px-3 py-2 text-sm"
							>
								<option value="non_profit">Non-profit</option>
								<option value="governmental">Governmental</option>
								<option value="other">Other</option>
							</select>
						</div>

						<div class="flex flex-col gap-2">
							<Label for="edit-contact">Contact Email</Label>
							<Input
								id="edit-contact"
								type="email"
								bind:value={editContactEmail}
								placeholder="contact@example.com"
							/>
						</div>

						<div class="flex flex-col gap-2">
							<Label for="edit-url">External URL</Label>
							<Input
								id="edit-url"
								type="url"
								bind:value={editExternalUrl}
								placeholder="https://example.com"
							/>
						</div>

						<div class="flex flex-col gap-2">
							<Label>Regions</Label>
							<div class="grid gap-2 rounded-md border p-3 md:grid-cols-2">
								{#if data.regions.length === 0}
									<p class="text-muted-foreground text-sm">
										No regions available.
									</p>
								{:else}
									{#each data.regions as region}
										<label class="flex items-center gap-2 text-sm">
											<input
												type="checkbox"
												checked={editRegionIds.includes(region.id)}
												onchange={(event) =>
													toggleRegion(
														region.id,
														(event.currentTarget as HTMLInputElement)
															.checked
													)}
											/>
											{region.name}
										</label>
									{/each}
								{/if}
							</div>
						</div>

						<div class="flex justify-end gap-2">
							<Button variant="outline" onclick={cancelEditing} disabled={isSaving}
								>Cancel</Button
							>
							<Button onclick={saveOrganization} disabled={isSaving}>
								{isSaving ? 'Saving...' : 'Save'}
							</Button>
						</div>
					</div>
				</div>
			{:else}
				<div class="grid gap-4 md:grid-cols-2">
					<div class="rounded-lg border p-4">
						<h2 class="text-lg font-semibold">Details</h2>
						<div class="mt-3 flex flex-col gap-2 text-sm">
							<p>
								<span class="font-medium">Organization ID:</span>
								{data.organization.id}
							</p>
							<p>
								<span class="font-medium">Type:</span>
								{data.organization.orgType}
							</p>
							{#if data.organization.mission}
								<p>
									<span class="font-medium">Mission:</span>
									{data.organization.mission}
								</p>
							{/if}
							{#if data.organization.regions.length > 0}
								<p>
									<span class="font-medium">Regions:</span>
									{data.organization.regions
										.map(
											(regionId) =>
												data.regions.find(
													(region) => region.id === regionId
												)?.name ?? regionId
										)
										.join(', ')}
								</p>
							{/if}
							{#if data.organization.contactEmail}
								<p>
									<span class="font-medium">Contact email:</span>
									<a
										href="mailto:{data.organization.contactEmail}"
										class="text-blue-600 hover:underline"
									>
										{data.organization.contactEmail}
									</a>
								</p>
							{/if}
							{#if data.organization.externalUrl}
								<p>
									<span class="font-medium">External URL:</span>
									<a
										href={data.organization.externalUrl}
										target="_blank"
										rel="noopener noreferrer"
										class="text-blue-600 hover:underline"
									>
										{data.organization.externalUrl}
									</a>
								</p>
							{/if}
						</div>
					</div>
				</div>
			{/if}
		{:else if activeTab === 'team'}
			{#if data.canManageTeam}
				<div class="rounded-lg border p-4">
					<h2 class="text-lg font-semibold">Members</h2>
					<p class="text-muted-foreground mt-1 text-sm">
						Add members and assign each member as member or admin.
					</p>

					<div class="mt-4 grid gap-2 md:grid-cols-[1fr_auto_auto]">
						<Input
							bind:value={memberEmail}
							placeholder="member@example.com"
							disabled={teamBusy}
						/>
						<select
							bind:value={newMemberRole}
							disabled={teamBusy}
							class="border-input bg-background h-10 rounded-md border px-3 py-2 text-sm"
						>
							<option value="member">Member</option>
							<option value="admin">Admin</option>
						</select>
						<Button
							onclick={() => addMember()}
							disabled={teamBusy || memberEmail.length === 0}>Add</Button
						>
					</div>

					<div class="mt-4 overflow-x-auto">
						<table class="w-full text-sm">
							<thead>
								<tr class="border-b text-left">
									<th class="py-2">Username</th>
									<th class="py-2">Email</th>
									<th class="py-2">Role</th>
									<th class="py-2 text-right">Actions</th>
								</tr>
							</thead>
							<tbody>
								{#each members as member (member.id)}
									{@const isCurrentUser = member.id === currentUserId}
									<tr class="border-b">
										<td class="py-2">
											<div class="flex items-center gap-2">
												<span>{member.username ?? 'Unknown user'}</span>
												{#if isCurrentUser}
													<span
														class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs"
													>
														You
													</span>
												{/if}
											</div>
										</td>
										<td class="py-2">{member.email ?? 'No email'}</td>
										<td class="py-2">
											<div class="flex flex-col gap-1">
												<select
													value={member.role}
													onchange={(event) =>
														updateMemberRole(
															member.id,
															(
																event.currentTarget as HTMLSelectElement
															).value as TeamRole
														)}
													disabled={teamBusy || isCurrentUser}
													title={isCurrentUser
														? 'You cannot change your own role'
														: undefined}
													class="border-input bg-background h-9 rounded-md border px-2 py-1 text-sm disabled:pointer-events-none disabled:opacity-50"
												>
													<option value="member">Member</option>
													<option value="admin">Admin</option>
												</select>
											</div>
										</td>
										<td class="py-2 text-right">
											<button
												type="button"
												aria-label="Remove member"
												class="hover:bg-primary group rounded-full p-1.5 disabled:pointer-events-none disabled:opacity-50"
												onclick={() => removeMember(member.id)}
												disabled={teamBusy || isCurrentUser}
											>
												<Trash
													class="group-hover:text-primary-foreground size-4"
												/>
											</button>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			{:else}
				<p class="text-muted-foreground">You do not have permission to manage this team.</p>
			{/if}
		{/if}
	{:else}
		<p class="text-muted-foreground">Organization not found.</p>
	{/if}
</div>

<AlertDialog.Root open={isDeleting}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete Organization</AlertDialog.Title>
			<AlertDialog.Description>
				Are you sure you want to delete <strong>{data.organization?.name}</strong>? This
				action cannot be undone.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<Button variant="outline" onclick={() => (isDeleting = false)}>Cancel</Button>
			<Button variant="destructive" onclick={deleteOrganization}>Delete</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<AlertDialog.Root open={confirmCreateUserOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Create new user account?</AlertDialog.Title>
			<AlertDialog.Description>
				No account exists for <strong>{pendingCreateUser?.email}</strong>. Continue to
				create a new email/password user, send an account setup email, and add them as
				<strong>{pendingCreateUser?.role ?? 'member'}</strong>?
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<Button
				variant="outline"
				onclick={() => {
					confirmCreateUserOpen = false;
					pendingCreateUser = null;
				}}
			>
				Cancel
			</Button>
			<Button onclick={confirmCreateUser} disabled={teamBusy}>Create user</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
