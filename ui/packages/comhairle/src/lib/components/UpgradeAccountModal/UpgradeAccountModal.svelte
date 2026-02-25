<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { PasswordInput } from '$lib/components/ui/password-input';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crown-shy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { upgradeAccountSchema } from './schema';
	import type { UserDto } from '@crown-shy/api-client/api';
	import { UserCheck, Mail, Lock } from 'lucide-svelte';
	import { invalidateAll } from '$app/navigation';

	let {
		open = $bindable(false),
		onSuccess = () => {},
		currentUser
	}: {
		open?: boolean;
		onSuccess?: (upgradedUser: UserDto) => void;
		currentUser: UserDto;
	} = $props();

	let saving = $state(false);

	const form = superForm(
		{
			username: currentUser?.username || '',
			email: '',
			password: '',
			confirmPassword: ''
		},
		{
			validators: zodClient(upgradeAccountSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: handleUpgrade
		}
	);

	const { form: formData, enhance, validateForm } = form;

	async function handleUpgrade() {
		const result = await validateForm({ update: true });
		if (!result.valid) return;

		try {
			saving = true;

			const upgradedUser = await apiClient.UpgradeAccount({
				username: result.data.username,
				email: result.data.email,
				password: result.data.password
			});

			notifications.send({
				message: 'Account upgraded successfully! You can now receive email notifications.',
				priority: 'SUCCESS'
			});

			open = false;
			invalidateAll();
			onSuccess(upgradedUser);
		} catch (error: any) {
			notifications.send({
				message:
					error?.response?.data?.message ||
					'Failed to upgrade account. Please try again.',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class="w-full">
		<Button class="w-full" variant="secondary">Upgrade Account</Button>
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title class="flex items-center gap-2">
				<UserCheck class="h-5 w-5" />
				Upgrade Your Account
			</Dialog.Title>
			<Dialog.Description>
				Transform your anonymous account into a full account to receive email notifications
				and keep track of your conversations. Your current participation will be preserved.
			</Dialog.Description>
		</Dialog.Header>

		<form method="POST" class="space-y-4" use:enhance>
			<Form.Field {form} name="username" class="space-y-2">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="flex items-center gap-2">
							<UserCheck class="h-4 w-4" />
							Username
						</Form.Label>
						<Input
							{...props}
							bind:value={$formData.username}
							placeholder="Choose a username"
							disabled={saving}
						/>
						<Form.FieldErrors />
					{/snippet}
				</Form.Control>
			</Form.Field>

			<Form.Field {form} name="email" class="space-y-2">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="flex items-center gap-2">
							<Mail class="h-4 w-4" />
							Email Address
						</Form.Label>
						<Input
							{...props}
							type="email"
							bind:value={$formData.email}
							placeholder="Enter your email address"
							disabled={saving}
						/>
						<Form.FieldErrors />
					{/snippet}
				</Form.Control>
			</Form.Field>

			<Form.Field {form} name="password" class="space-y-2">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="flex items-center gap-2">
							<Lock class="h-4 w-4" />
							Password
						</Form.Label>
						<PasswordInput
							{...props}
							bind:value={$formData.password}
							placeholder="Create a secure password"
							disabled={saving}
						/>
						<Form.FieldErrors />
					{/snippet}
				</Form.Control>
			</Form.Field>

			<Form.Field {form} name="confirmPassword" class="space-y-2">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="flex items-center gap-2">
							<Lock class="h-4 w-4" />
							Confirm Password
						</Form.Label>
						<PasswordInput
							{...props}
							bind:value={$formData.confirmPassword}
							placeholder="Confirm your password"
							disabled={saving}
						/>
						<Form.FieldErrors />
					{/snippet}
				</Form.Control>
			</Form.Field>

			<Dialog.Footer class="flex justify-between gap-2 pt-4">
				<Button variant="outline" onclick={() => (open = false)} disabled={saving}
					>Cancel</Button
				>
				<Button type="submit" disabled={saving} class="flex items-center gap-2">
					{#if saving}
						Upgrading...
					{:else}
						<UserCheck class="h-4 w-4" />
						Upgrade Account
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
