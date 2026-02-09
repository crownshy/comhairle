<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import type { PageProps } from './$types';
	import * as m from '$lib/paraglide/messages';
	import * as Form from '$lib/components/ui/form';
	import Button from '$lib/components/ui/button/button.svelte';
	import PasswordInput from '$lib/components/ui/password-input/password-input.svelte';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import { passwordResetUpdateFormSchema } from '$lib/profile/schema';
	import { apiClient } from '$lib/api/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { useLoading } from '$lib/hooks/use-loading.svelte';

	let { data }: PageProps = $props();

	const form = superForm(defaults(zod(passwordResetUpdateFormSchema)), {
		validators: zodClient(passwordResetUpdateFormSchema),
		taintedMessage: false,
		onSubmit: passwordResetUpdate
	});

	const { form: formData, errors, validateForm, enhance } = form;
	let responseMessage: string | null = $state(null);
	const loader = useLoading();

	async function passwordResetUpdate() {
		let result = await validateForm({ update: true });
		if (result.valid) {
			let { password, confirmPassword } = result.data;
			await loader.run(async () => {
				try {
					await apiClient.PasswordResetUpdate({
						password,
						confirm_password: confirmPassword,
						token: data.token
					});
					await invalidateAll();
					await goto(resolve('/auth/password-reset/success'));
				} catch (e) {
					responseMessage = m.something_went_wrong();
				}
			});
		}
	}
</script>

<AuthPage>
	<section>
		<form class="space-y-4" use:enhance method="POST">
			<h1 class="text-xl font-bold">{m.create_new_password_heading()}</h1>
			{#if responseMessage}
				<p class="text-destructive text-sm">{responseMessage}</p>
			{/if}
			<Form.Field {form} name="password">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>{m.new_password()}</Form.Label>
						<PasswordInput
							bind:value={$formData.password}
							{...props}
							placeholder={m.new_password()}
						/>
					{/snippet}
				</Form.Control>
			</Form.Field>
			{#if $errors.password}
				<p class="text-destructive text-sm">{$errors.password}</p>
			{/if}
			<Form.Field {form} name="confirmPassword">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>{m.confirm_password()}</Form.Label>
						<PasswordInput
							bind:value={$formData.confirmPassword}
							{...props}
							placeholder={m.confirm_password()}
						/>
					{/snippet}
				</Form.Control>
			</Form.Field>
			{#if $errors.confirmPassword}
				<p class="text-destructive text-sm">{$errors.confirmPassword}</p>
			{/if}
			<Button type="submit">
				{#if loader.loading}
					<span class="flex w-12 justify-center">
						<Spinner />
					</span>
				{:else}
					{m.submit()}
				{/if}
			</Button>
		</form>
	</section>
</AuthPage>
