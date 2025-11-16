<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { userDetailsSchema } from './schema';
	import type { User } from '$lib/api/api';
	import { onMount } from 'svelte';

	let {
		user
	}: {
		user: User | null;
	} = $props();

	let loading = $state(false);
	let saving = $state(false);

	const form = superForm(
		{
			username: user?.username || '',
			password: '',
			confirmPassword: ''
		},
		{
			validators: zodClient(userDetailsSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: saveUserDetails
		}
	);

	const { form: formData, enhance, validateForm, errors } = form;

	async function saveUserDetails() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

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
			
			// Clear password fields after successful update
			$formData.password = '';
			$formData.confirmPassword = '';
			
			notifications.send({
				message: 'User details updated successfully!',
				priority: 'SUCCESS'
			});
			
		} catch (error: any) {
			notifications.send({
				message: error?.response?.data?.message || 'Failed to update user details. Please try again.',
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

	<form method="POST" class="space-y-4" use:enhance>
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
					<Input
						{...props}
						type="password"
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
					<Input
						{...props}
						type="password"
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
</div>