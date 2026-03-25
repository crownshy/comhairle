<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { PasswordInput } from '$lib/components/ui/password-input';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { userDetailsSchema } from './schema';
	import type { User } from '@crownshy/api-client/api';
	import { onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages.js';
	import { goto } from '$app/navigation';

	let {
		user
	}: {
		user: User | null;
	} = $props();

	let loading = $state(false);
	let saving = $state(false);
	let showSuccessDialog = $state(false);

	const form = superForm(
		{
			username: user?.username || '',
			password: '',
			confirmPassword: ''
		},
		{
			validators: zodClient(userDetailsSchema),
			taintedMessage: false,
			validationMethod: 'onsubmit'
		}
	);

	const { form: formData, validateForm, errors } = form;

	async function saveUserDetails(e: Event) {
		e.preventDefault();
		const result = await validateForm({ update: false });
		if (!result.valid) {
			$errors = result.errors;
			return;
		}

		try {
			saving = true;

			// Prepare the update data, excluding confirmPassword and empty values
			const updateData: { username?: string; password?: string } = {};

			if (result.data.username && result.data.username !== user?.username) {
				updateData.username = result.data.username;
			}

			if (result.data.password) {
				updateData.password = result.data.password;
			}

			// Only make the API call if there are changes to save
			if (Object.keys(updateData).length === 0) {
				notifications.send({
					message: 'No changes to save.',
					priority: 'INFO'
				});
				return;
			}

			const updatedUser = await apiClient.UpdateUserDetails(updateData);

			// Clear validation errors first, then password fields
			$errors = {};
			$formData.password = '';
			$formData.confirmPassword = '';

			showSuccessDialog = true;
		} catch (error: any) {
			notifications.send({
				message:
					error?.response?.data?.err ||
					error?.response?.data?.message ||
					'Failed to update user details. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

<div class="space-y-6">
	<div>
		<h3 class="text-lg font-medium">Account Details</h3>
		<p class="text-muted-foreground text-sm">Update your username and password here.</p>
	</div>

	<form method="POST" class="space-y-4" onsubmit={saveUserDetails}>
		<Form.Field {form} name="username" class="space-y-2">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Username</Form.Label>
					<Input
						{...props}
						bind:value={$formData.username}
						placeholder="Enter your username"
						disabled={saving}
					/>
					<Form.FieldErrors />
				{/snippet}
			</Form.Control>
		</Form.Field>

		<Form.Field {form} name="password" class="space-y-2">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>New Password</Form.Label>
					<PasswordInput
						{...props}
						bind:value={$formData.password}
						placeholder="Enter new password (optional)"
						disabled={saving}
					/>

					<Form.FieldErrors />
				{/snippet}
			</Form.Control>
		</Form.Field>

		<Form.Field {form} name="confirmPassword" class="space-y-2">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Confirm New Password</Form.Label>
					<PasswordInput
						{...props}
						bind:value={$formData.confirmPassword}
						placeholder="Confirm new password"
						disabled={saving || !$formData.password}
					/>
					<Form.FieldErrors />
				{/snippet}
			</Form.Control>
		</Form.Field>

		<div class="flex justify-end">
			<Button type="submit" disabled={saving}>
				{saving ? 'Saving...' : 'Save Changes'}
			</Button>
		</div>
	</form>

	<Dialog.Root bind:open={showSuccessDialog}>
		<Dialog.Content class="sm:max-w-[425px]">
			<Dialog.Header>
				<Dialog.Title>{m.password_updated_successfully()}</Dialog.Title>
				<Dialog.Description>
					{m.password_updated_successfully_body()}
				</Dialog.Description>
			</Dialog.Header>
			<Dialog.Footer>
				<Button onclick={() => goto('/')}>{m.continue_to_home()}</Button>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>
</div>
