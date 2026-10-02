<script lang="ts">
	import type { Snapshot } from '@crownshy/api-client/api';
	import { LoaderCircle } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Badge } from '$lib/components/ui/badge';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Dialog from '$lib/components/ui/dialog';
	import { snakeToStartCase } from '$lib/utils/casingUtils';
	import type { PageData } from './$types';
	import type { RoleRecipient } from './roleAssignments';

	type Props = {
		open: boolean;
		recipient: RoleRecipient | null;
		snapshot: Snapshot | null;
		selectedRoles: string[];
		loading: boolean;
		saving: boolean;
		error: string;
		newKind: 'user' | 'organization';
		email: string;
		organizationId: string;
		canAdmin: boolean;
		roleData: NonNullable<NonNullable<PageData['roleManagement']>['ok']> | null;
		hasChanges: boolean;
		onToggleRole: (role: string, checked: boolean) => void;
		onSave: () => Promise<void>;
		onReload: (recipient: RoleRecipient) => Promise<void>;
	};

	let {
		open = $bindable(),
		recipient,
		snapshot,
		selectedRoles,
		loading,
		saving,
		error,
		newKind = $bindable(),
		email = $bindable(),
		organizationId = $bindable(),
		canAdmin,
		roleData,
		hasChanges,
		onToggleRole,
		onSave,
		onReload
	}: Props = $props();
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="flex max-h-[90dvh] flex-col overflow-hidden sm:max-w-lg"
		showCloseButton={!saving}
		onInteractOutside={(event) => {
			if (saving) event.preventDefault();
		}}
		onEscapeKeydown={(event) => {
			if (saving) event.preventDefault();
		}}
	>
		<Dialog.Header class="pr-6">
			<Dialog.Title
				>{recipient ? `Edit roles for ${recipient.name}` : 'Add recipient'}</Dialog.Title
			>
			<Dialog.Description>{recipient?.kind ?? 'Role assignment'}</Dialog.Description>
		</Dialog.Header>
		<form
			class="flex min-h-0 flex-1 flex-col gap-6"
			onsubmit={(event) => {
				event.preventDefault();
				onSave();
			}}
		>
			<div class="min-h-0 flex-1 space-y-6 overflow-y-auto">
				{#if !recipient}
					<div class="space-y-2">
						<Label for="recipient-type">Type</Label>
						<select
							id="recipient-type"
							bind:value={newKind}
							disabled={saving || !canAdmin}
							class="border-input bg-background h-10 w-full rounded-md border px-3 text-base"
						>
							<option value="user">User</option>
							<option value="organization">Organization</option>
						</select>
					</div>
					{#if newKind === 'user'}<div class="space-y-2">
							<Label for="recipient-email">Email</Label>
							<Input
								id="recipient-email"
								type="email"
								bind:value={email}
								required
								disabled={saving || !canAdmin}
							/>
						</div>
					{:else}<div class="space-y-2">
							<Label for="recipient-organization">Organization</Label>
							<select
								id="recipient-organization"
								bind:value={organizationId}
								required
								disabled={saving || !canAdmin}
								class="border-input bg-background h-10 w-full rounded-md border px-3 text-base"
							>
								<option value="" disabled>Select organization</option>
								{#each roleData?.organizations ?? [] as organization (organization.id)}
									<option value={organization.id}>{organization.name}</option>
								{/each}
							</select>
						</div>{/if}
				{/if}
				{#if loading}<p class="flex items-center gap-2 text-base" role="status">
						<LoaderCircle class="size-4 animate-spin" /> Loading roles
					</p>
				{:else}
					<fieldset
						disabled={!canAdmin || saving || (recipient !== null && snapshot === null)}
						class="space-y-3"
					>
						<legend class="mb-3 text-base font-semibold">Assign roles</legend>
						{#each roleData?.roles ?? [] as role (role)}
							<div class="flex items-center gap-3">
								<Checkbox
									id={`assignment-${role}`}
									checked={selectedRoles.includes(role)}
									disabled={!canAdmin ||
										saving ||
										(recipient !== null && snapshot === null)}
									onCheckedChange={(checked) => onToggleRole(role, checked)}
								/>
								<Label for={`assignment-${role}`} class="text-base">
									{snakeToStartCase(role)}
								</Label>
							</div>
						{/each}
					</fieldset>
				{/if}
				{#if snapshot?.inherited.length}<div class="space-y-3">
						<h4 class="text-base font-semibold">Inherited roles</h4>
						{#each snapshot.inherited as inherited (inherited.group_id)}<div>
								<p class="text-muted-foreground text-base">
									{inherited.group_name}
								</p>
								<div class="mt-1 flex flex-wrap gap-1">
									{#each inherited.roles as role (role)}<Badge
											variant="outline"
											class="text-base">{snakeToStartCase(role)}</Badge
										>{/each}
								</div>
							</div>{/each}
					</div>{/if}
				{#if error}<p class="text-destructive text-base" role="alert">{error}</p>
					{#if recipient}<Button
							type="button"
							variant="outline"
							disabled={saving || !canAdmin}
							onclick={() => recipient && onReload(recipient)}
							>Reload assignments</Button
						>{/if}{/if}
			</div>
			<Dialog.Footer class="border-border shrink-0 border-t pt-4">
				<Button
					type="button"
					variant="outline"
					disabled={saving}
					onclick={() => (open = false)}>Cancel</Button
				>
				<Button
					type="submit"
					disabled={!canAdmin ||
						loading ||
						saving ||
						!hasChanges ||
						(!recipient && selectedRoles.length === 0) ||
						(recipient !== null && snapshot === null)}
				>
					{#if saving}
						<LoaderCircle class="size-4 animate-spin" />
					{/if}
					Save changes
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
