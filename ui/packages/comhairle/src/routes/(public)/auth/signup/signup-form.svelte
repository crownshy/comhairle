<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { PasswordInput } from '$lib/components/ui/password-input';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import { signupFormSchema } from '$lib/profile';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';

	let { backTo } = $props();
	let responseMessage = $state(null);
	const loader = useLoading();

	const form = superForm(defaults(zod(signupFormSchema)), {
		validators: zodClient(signupFormSchema),
		taintedMessage: false,
		onSubmit: attemptLogin
	});

	let { form: formData, validateForm, enhance } = form;

	async function attemptLogin() {
		let result = await validateForm({ update: true });
		if (result.valid) {
			let { username, password, email } = result.data;
			await loader.run(async () => {
				try {
					const user = await apiClient.SignUp({
						username,
						password,
						email
					});
					await invalidateAll();
					if (user.auth_type === 'annon') {
						await goto(backTo ?? '/');
					} else {
						await goto('/auth/verification-message');
					}
				} catch (e) {
					responseMessage = e.response.data.err;
				}
			});
		}
	}
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.create_an_account()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{m.get_started_with_comhairle_today()}</p>
	</div>
	{#if responseMessage}
		<p class="text-destructive text-sm">{responseMessage}</p>
	{/if}
	<Form.Field {form} name="username">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.username()}</Form.Label>
				<Input {...props} bind:value={$formData.username} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="email">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.email()}</Form.Label>
				<Input {...props} bind:value={$formData.email} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.new_password()}</Form.Label>
				<PasswordInput {...props} bind:value={$formData.password} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password_confirm">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.confirm_password()}</Form.Label>
				<PasswordInput {...props} bind:value={$formData.password_confirm} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<LoadingButton type="submit" class="w-full" variant="default" loading={loader.loading}>
		{m.submit()}
	</LoadingButton>
	<a
		href={`/auth/anonymous-signup?backTo=${backTo}`}
		class={cn('w-full', buttonVariants({ variant: 'outline' }))}>{m.sign_up_anonymously()}</a
	>
	<p class="text-sm">
		<a href={`/auth/login?backTo=${backTo}`}>{m.already_have_an_account_login()}</a>
	</p>

	<p class="text-muted-foreground mb-4 text-sm">
		{m.agree_to_tos()}
		<a href="/rights/tos">TOS</a>
		{m.agree_to_tos2()}
		<a href="/rights/privacy">{m.agree_to_tos_privacy()}</a>
	</p>
</form>
