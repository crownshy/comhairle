<script lang="ts">
	import { resolve } from '$app/paths';
	import * as Form from '$lib/components/ui/form';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import Input from '$lib/components/ui/input/input.svelte';
	import { LoadingButton } from '$lib/components/ui/button';
	import { passwordResetCreateFormSchema } from '$lib/profile/schema';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { useLoading } from '$lib/hooks/use-loading.svelte';

	let responseMessage: string | null = $state(null);


	const form = superForm(defaults(zod(passwordResetCreateFormSchema)), {
		validators: zodClient(passwordResetCreateFormSchema),
		taintedMessage: false,
		onSubmit: passwordResetCreate
	});
	const { form: formData, errors, validateForm, enhance } = form;
	const loader = useLoading();

	async function passwordResetCreate() {
		let result = await validateForm({ update: true });
		if (result.valid) {
			let { email } = result.data;
			await loader.run(async () => {
				try {
					await apiClient.PasswordResetCreate({ email });
					await invalidateAll();
					await goto(resolve('/auth/password-reset/sent'));
				} catch (e) {
					if (e.response?.status === 404) {
						responseMessage = m.email_address_not_found();
					} else {
						responseMessage = m.something_went_wrong();
					}
				}
			});
		}
	}
</script>

<svelte:head>
	<title>Reset Password - Comhairle</title>
</svelte:head>

<AuthPage>
	<section>
		<form class="space-y-4" method="POST" use:enhance>
			<h1 class="text-xl font-bold">{m.reset_password_heading()}</h1>
			<p class="text-muted-foreground mb-4 text-sm">{m.reset_password_body()}</p>
			{#if responseMessage}
				<p class="text-destructive text-sm">{responseMessage}</p>
			{:else if $errors.email}
				<p class="text-destructive text-sm">{$errors.email}</p>
			{/if}
			<Form.Field {form} name="email">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>{m.email()}</Form.Label>
						<Input
							bind:value={$formData.email}
							placeholder={m.email_placeholder()}
							{...props}
							required
						/>
					{/snippet}
				</Form.Control>
			</Form.Field>
			<LoadingButton type="submit" loading={loader.loading}>
				{m.submit()}
			</LoadingButton>
		</form>
	</section>
</AuthPage>
