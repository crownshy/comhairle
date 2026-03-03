<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { loginFormSchema } from '$lib/profile';
	import { superForm, defaults } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import PasswordInput from '$lib/components/ui/password-input/password-input.svelte';
	import { resolve } from '$app/paths';

	let { backTo }: { backTo?: string } = $props();

	const form = superForm(defaults(zod(loginFormSchema)), {
		validators: zodClient(loginFormSchema),
		taintedMessage: false,
		onSubmit: attemptLogin
	});

	let responseMessage = $state(null);
	const loader = useLoading();

	const { form: formData, enhance, validateForm } = form;

	async function attemptLogin() {
		let result = await validateForm({ update: true });
		if (result.valid) {
			let { email, password } = result.data;
			await loader.run(async () => {
				try {
					await apiClient.LoginUser({
						email,
						password
					});
					await invalidateAll();

					let redirectTo = backTo ?? '/';
					if (redirectTo === '/') {
						try {
							const userRoles = await apiClient.GetUserRoles();
							const isAdmin = userRoles
								?.find((ur) => ur.resource === 'Site')
								?.roles.includes('Admin');
							if (isAdmin) {
								redirectTo = '/admin';
							}
						} catch {}
					}

					await goto(resolve(redirectTo));
				} catch (e) {
					responseMessage = e.response.data.err;
				}
			});
		}
	}
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.login()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{m.enter_your_details_below_to_login()}</p>
	</div>

	{#if responseMessage}
		<p class="text-destructive text-sm">{responseMessage}</p>
	{/if}

	<Form.Field {form} name="email">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.email()}</Form.Label>
				<Input {...props} bind:value={$formData.email} required />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="password">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.password()}</Form.Label>
				<PasswordInput bind:value={$formData.password} {...props} required />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<LoadingButton type="submit" class="w-full" variant="default" loading={loader.loading}>
		{m.submit()}
	</LoadingButton>

	<Button href={`/auth/anonymous-login?backTo=${backTo ?? '/'}`} variant="link" class="w-full">
		{m.login_with_anonymous_id()}
	</Button>

	<p class="text-sm">
		<a href={resolve(`/auth/password-reset/create`)} class="hover:underline"
			>{m.forgotten_password()}</a
		>
	</p>

	<p class="text-sm">
		<a href={resolve(`/auth/signup?backTo=${backTo ?? '/'}`)} class="hover:underline"
			>{m.dont_have_an_account_signup()}</a
		>
	</p>
</form>
