<script lang="ts">
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Card from '$lib/components/ui/card';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import * as Form from '$lib/components/ui/form/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import Input from '../ui/input/input.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import { userEmailPermissionsForm } from './schema';
	import { defaults, superForm } from 'sveltekit-superforms';
	import Spinner from '../ui/spinner/spinner.svelte';
	import { UserWithPermissionDto } from '@crownshy/api-client/api';
	import { invalidate } from '$app/navigation';
	import { Trash, LoaderCircle } from 'lucide-svelte';

	type Props = {
		resourceId: string;
		resourceType: string;
		role: string;
		grantReason: string;
		permittedUsers: UserWithPermissionDto[];
	};

	let { resourceId, resourceType, role, grantReason, permittedUsers }: Props = $props();

	let loading = $state(false);

	let userEmailPermissionForm = superForm(defaults(zod(userEmailPermissionsForm)), {
		validators: zodClient(userEmailPermissionsForm),
		taintedMessage: false,
		validationMethod: 'oninput',
		onSubmit: attemptGrantUserPermission
	});

	let { form, enhance, validateForm, message, submitting } = userEmailPermissionForm;

	async function attemptGrantUserPermission({ cancel }: { cancel: () => void }) {
		cancel();
		const result = await validateForm({ update: true });
		if (!result.valid) {
			return;
		}

		loading = true;

		const response = await tryCatchAsync(() =>
			apiClient.GrantPermission(
				{
					user_email: result.data.email as string,
					role_name: role,
					grant_reason: grantReason
				},
				{ params: { resource_type: resourceType, resource_id: resourceId } }
			)
		);

		if (response.err !== null) {
			let errorMessage = `Something went wrong granting ${snakeToSentenceCase(role)} permission for this ${snakeToSentenceCase(resourceType)}.`;
			if (response.err.response?.status === 409) {
				errorMessage +=
					' This user / organization may already have this permission granted.';
			}
			if (response.err.response?.status === 404) {
				errorMessage += ' Could not find this user / organization.';
			}
			console.error(response.err);
			notifications.send({
				message: errorMessage,
				priority: 'ERROR'
			});

			loading = false;
			return;
		}

		loading = false;
		notifications.send({
			message: `Successfully granted ${snakeToSentenceCase(role)} permission`
		});
		invalidate('conversation:meta');
	}

	let selectedUserId = $state<string | null>(null);
	let openRevokeDialog = $state(false);
	let revoking = $state(false);
	async function handleRevoke() {
		try {
			apiClient.RevokePermission(undefined, {
				params: { resource_type: resourceType, resource_id: resourceId },
				queries: { user_id: selectedUserId, role_name: role }
			});

			notifications.send({
				message: 'Permission successfully revoked',
				priority: 'INFO'
			});

			invalidate('conversation:meta');
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Something went wrong revoking this permission',
				priority: 'ERROR'
			});
		}
		resetRevoke();
	}

	function resetRevoke() {
		selectedUserId = null;
		openRevokeDialog = false;
	}
</script>

<section class="flex flex-col gap-8 py-8">
	<h3 class="text-xl font-semibold">Permissions</h3>

	<form method="POST" class="flex flex-col gap-4" use:enhance>
		<Form.Field form={userEmailPermissionForm} name="email" class="flex flex-col gap-2">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						>Grant a user "{snakeToSentenceCase(role).toLowerCase()}" access to this {snakeToSentenceCase(
							resourceType
						).toLowerCase()}</Form.Label
					>
					<Input bind:value={$form.email} placeholder="Email address" {...props} />
				{/snippet}
			</Form.Control>
		</Form.Field>

		<Form.Button type="submit" disabled={$submitting}>
			Submit
			{#if loading}
				<Spinner />
			{/if}
		</Form.Button>

		{#if message}
			<p>{$message}</p>
		{/if}
	</form>
	<Card.Root>
		<Card.Header>
			<h1 class="text-xl font-bold">
				{`${snakeToSentenceCase(resourceType)} ${snakeToSentenceCase(role).toLowerCase()} users`}
			</h1>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-37.5">Username</Table.Head>
						<Table.Head class="w-37.5">Email</Table.Head>
						<Table.Head class="w-37.5">Role</Table.Head>
						<Table.Head class="w-37.5">Actions</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each permittedUsers as user (user.id)}
						<Table.Row>
							<Table.Cell>{user.username}</Table.Cell>
							<Table.Cell>{user.email}</Table.Cell>
							<Table.Cell>{user.roleName}</Table.Cell>
							<Table.Cell>
								<button
									type="button"
									aria-label="Revoke permission"
									class="hover:bg-primary group rounded-full p-1.5 transition-all duration-200 ease-in-out"
									onclick={() => {
										selectedUserId = user.id;
										openRevokeDialog = true;
									}}
								>
									<Trash
										class="group-hover:text-primary-foreground transition-all duration-200 ease-in-out"
									/>
								</button>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</Card.Content>
	</Card.Root>
</section>

<AlertDialog.Root open={openRevokeDialog}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title
				>Revoke "{snakeToSentenceCase(role).toLowerCase()}" permission?</AlertDialog.Title
			>
			<AlertDialog.Description>
				This will permanently remove this permission for this user.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={revoking} onclick={() => resetRevoke()}
				>Cancel</AlertDialog.Cancel
			>
			<AlertDialog.Action disabled={revoking} onclick={handleRevoke}>
				{#if revoking}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/if}
				Revoke
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
