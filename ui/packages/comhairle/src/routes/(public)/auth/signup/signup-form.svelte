<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { PasswordInput } from '$lib/components/ui/password-input';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import { signupFormSchema } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';

	let { backTo } = $props();
	let responseMessage = $state(null);
	const loader = useLoading();

	const form = superForm(defaults(zod(signupFormSchema)), {
		validators: zodClient(signupFormSchema),
		taintedMessage: false,
		onSubmit: async ({ cancel }) => {
			cancel();
			await attemptLogin();
		}
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
						await goto(
							`/auth/verification-message?backTo=${encodeURIComponent(backTo ?? '/')}`
						);
					}
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
			{m.create_an_account()}
		</h1>
		<p
			class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
		>
			{m.signup_subtitle()}
		</p>
	</div>

	{#if responseMessage}
		<p class="text-destructive text-center text-sm">{responseMessage}</p>
	{/if}

	<div class="space-y-6">
		<Form.Field {form} name="username">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.username()}</Form.Label>
					<Input
						{...props}
						placeholder={m.username()}
						bind:value={$formData.username}
						required
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="email">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.email()}</Form.Label>
					<Input {...props} placeholder={m.email()} bind:value={$formData.email} />
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="password">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.password()}</Form.Label>
					<PasswordInput
						{...props}
						placeholder={m.please_enter_a_password()}
						bind:value={$formData.password}
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field {form} name="password_confirm">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.confirm_password()}</Form.Label>
					<PasswordInput
						{...props}
						placeholder={m.confirm_password()}
						bind:value={$formData.password_confirm}
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
			{m.sign_up()}
		</LoadingButton>

		<Button
			href={`/auth/anonymous-signup?backTo=${encodeURIComponent(backTo ?? '/')}`}
			variant="outline"
			size="lg"
			class="h-12 w-full px-7 lg:w-auto"
		>
			{m.sign_up_anonymously()}
		</Button>
	</div>

	<div class="flex flex-col gap-1 font-light">
		<p class="text-muted-foreground text-base">
			{m.already_have_an_account_login().split('?')[0]}?
			<a
				href={`/auth/login?backTo=${encodeURIComponent(backTo ?? '/')}`}
				class="text-primary underline">{m.login()}</a
			>
		</p>
		<p class="text-muted-foreground text-base">
			{m.agree_to_tos()}
			<a href="/rights/tos" class="text-primary underline">TOS</a>
			{m.agree_to_tos2()}
			<a href="/rights/privacy" class="text-primary underline">{m.agree_to_tos_privacy()}</a>
		</p>
	</div>
</form>
