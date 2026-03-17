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

<form class="space-y-6 lg:space-y-8" method="POST" use:enhance>
	<div class="flex flex-col items-center gap-3 lg:gap-6">
		<h1
			class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-[52px]"
		>
			{m.login()}
		</h1>
		<p
			class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
		>
			{m.good_to_see_you_back()}
		</p>
	</div>

	{#if responseMessage}
		<p class="text-destructive text-center text-sm">{responseMessage}</p>
	{/if}

	<div class="space-y-6">
		<Form.Field {form} name="email">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.email()}</Form.Label>
					<Input
						{...props}
						placeholder={m.email()}
						bind:value={$formData.email}
						required
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="password">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.password()}</Form.Label>
					<PasswordInput
						bind:value={$formData.password}
						placeholder={m.please_enter_a_password()}
						{...props}
						required
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
	</div>

	<div class="flex flex-col gap-3 lg:flex-row lg:flex-wrap lg:items-center lg:gap-4">
		<LoadingButton
			type="submit"
			size="lg"
			class="h-12 w-full px-7 lg:w-auto"
			variant="default"
			loading={loader.loading}
		>
			{m.login()}
		</LoadingButton>

		<Button
			href={resolve(`/auth/anonymous-login?backTo=${backTo ?? '/'}`)}
			variant="outline"
			size="lg"
			class="h-12 w-full px-7 lg:w-auto"
		>
			{m.login_with_anonymous_id()}
		</Button>
	</div>

	<div class="flex flex-col gap-1">
		<a href={resolve(`/auth/password-reset/create`)} class="text-primary text-base underline">
			{m.forgotten_password()}
		</a>
		<p class="text-muted-foreground text-base">
			{m.dont_have_an_account_signup().split('?')[0]}?
			<a
				href={resolve(`/auth/signup?backTo=${backTo ?? '/'}`)}
				class="text-primary underline"
			>
				{m.signup()}
			</a>
		</p>
	</div>
</form>
