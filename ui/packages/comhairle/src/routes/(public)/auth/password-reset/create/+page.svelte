<script lang="ts">
	import { resolve } from '$app/paths';
	import * as Form from '$lib/components/ui/form';

	import * as m from '$lib/paraglide/messages';
	import Input from '$lib/components/ui/input/input.svelte';
	import { LoadingButton } from '$lib/components/ui/button';
	import { passwordResetCreateFormSchema } from '$lib/profile/schema';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { useLoading } from '$lib/hooks/use-loading.svelte';
	import AuthGradient from '$lib/components/AuthGradient.svelte';

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

<div class="flex min-h-screen w-full flex-col">
	<AuthGradient showLogo={false} mode="full">
		<div
			class="dark:bg-card mx-4 w-full max-w-lg rounded-xl bg-white p-6 shadow-sm lg:mx-0 lg:p-10"
		>
			<form class="space-y-8" method="POST" use:enhance>
				<div class="flex flex-col items-center gap-3 lg:gap-6">
					<h1
						class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-[52px]"
					>
						{m.reset_password_heading()}
					</h1>
					<p
						class="text-muted-foreground text-center text-base leading-6 font-semibold lg:text-2xl lg:leading-7"
					>
						{m.reset_password_body()}
					</p>
				</div>

				{#if responseMessage}
					<p class="text-destructive text-center text-sm">{responseMessage}</p>
				{:else if $errors.email}
					<p class="text-destructive text-center text-sm">{$errors.email}</p>
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

				<div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:gap-4">
					<LoadingButton
						type="submit"
						size="lg"
						class="h-12 w-full px-7 lg:w-auto"
						loading={loader.loading}
					>
						{m.submit()}
					</LoadingButton>
				</div>
			</form>
		</div>
	</AuthGradient>
</div>
