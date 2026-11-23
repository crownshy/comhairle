<script lang="ts">
	import { canAccessAdminPortal } from '$lib/utils/permissions';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { signupOtpSchema } from '$lib/profile';
	import { superForm, defaults } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import { m } from '$lib/paraglide/messages';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { key } from '$lib/utils/invalidationKey';

	let { backTo }: { backTo?: string } = $props();

	const form = superForm(defaults(zod(signupOtpSchema)), {
		validators: zodClient(signupOtpSchema),
		taintedMessage: false,
		onSubmit: async ({ cancel }) => {
			cancel();
			await attemptOtpSignup();
		}
	});

	let responseMessage = $state(null);
	const loader = useLoading();

	const { form: formData, enhance, validateForm } = form;

	async function attemptOtpSignup() {
		let result = await validateForm({ update: true });
		if (result.valid) {
			let { email, username } = result.data;
			await loader.run(async () => {
				try {
					await apiClient.SignupOtp({
						email,
						username
					});

					let redirectTo = backTo ?? '/';
					if (redirectTo === '/') {
						try {
							const userRoles = await apiClient.GetUserRoles();
							const isAdmin = canAccessAdminPortal(userRoles);
							if (isAdmin) {
								redirectTo = '/admin';
							}
						} catch {}
					}

					await goto(resolve(redirectTo), { invalidate: [key('user')] });
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
			class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-13"
		>
			{m.create_an_account()}
		</h1>
		<h2
			class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
		>
			{m.signup_subtitle()}
		</h2>
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
</form>

<div class="spacing-y-6 lg:spacing-y-8 mt-6 lg:mt-8">
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
</div>
