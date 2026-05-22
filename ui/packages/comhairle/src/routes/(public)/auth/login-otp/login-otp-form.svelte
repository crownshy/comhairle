<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { loginOtpSchema, sendOtpSchema } from '$lib/profile';
	import { superForm, defaults } from 'sveltekit-superforms';
	import { zod, zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { Spinner } from '$lib/components/ui/spinner';
	import { onMount } from 'svelte';

	let { backTo }: { backTo?: string } = $props();

	let email = $state<string | null>(null);

	const form = superForm(defaults(zod(loginOtpSchema)), {
		validators: zodClient(loginOtpSchema),
		taintedMessage: false,
		onSubmit: async ({ cancel }) => {
			cancel();
			await attemptOtpLogin();
		}
	});

	let responseMessage = $state(null);
	const loader = useLoading();

	const { form: formData, enhance, validateForm } = form;

	onMount(() => {
		if (!sessionStorage) return;
		if (!email) {
			email = sessionStorage?.getItem('pendingOtpEmail');
			sessionStorage?.clear();
		}
	});

	async function attemptOtpLogin() {
		let result = await validateForm({ update: true });
		if (result.valid && email) {
			let { code } = result.data;
			await loader.run(async () => {
				try {
					await apiClient.LoginOtpUser({
						email,
						code
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

	const resendForm = superForm(email ? { email } : defaults(zod(sendOtpSchema)), {
		validators: zodClient(sendOtpSchema),
		taintedMessage: false,
		onSubmit: async ({ cancel }) => {
			cancel();
			await sendOtp();
		}
	});

	const { enhance: resendEnhance, validateForm: resendValidateForm } = resendForm;
	const resendLoader = useLoading();

	async function sendOtp() {
		let result = await resendValidateForm({ update: true });
		if (result.valid && email) {
			let { email } = result.data;
			await resendLoader.run(async () => {
				try {
					await apiClient.CreateOtp({
						email
					});
					await invalidateAll();
				} catch (e) {
					responseMessage = e.response?.data?.err ?? 'Failed to send one-time-passcode';
				}
			});
		}
	}
</script>

{#if email}
	<form class="space-y-6 lg:space-y-8" method="POST" use:enhance>
		<div class="flex flex-col items-center gap-3 lg:gap-6">
			<h1
				class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-13"
			>
				{m.login_with_otp()}
			</h1>
			<p
				class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
			>
				{m.login_with_otp_descripton()}
			</p>
		</div>

		{#if responseMessage}
			<p class="text-destructive text-center text-sm">{responseMessage}</p>
		{/if}

		<div class="space-y-6">
			<span class="inline-block">{email}</span>

			<Form.Field {form} name="code">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>{m.otp_placeholder()}</Form.Label>
						<Input
							{...props}
							placeholder={m.otp_placeholder()}
							bind:value={$formData.code}
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
				href={resolve(`/auth/login?backTo=${encodeURIComponent(backTo ?? '/')}`)}
				variant="outline"
				size="lg"
				class="h-12 w-full px-7 lg:w-auto"
			>
				{m.login_with_password()}
			</Button>

			<Button
				href={resolve(`/auth/anonymous-login?backTo=${encodeURIComponent(backTo ?? '/')}`)}
				variant="outline"
				size="lg"
				class="h-12 w-full px-7 lg:w-auto"
			>
				{m.login_with_anonymous_id()}
			</Button>
		</div>
	</form>

	<div class="spacing-y-6 lg:spacing-y-8 mt-6 lg:mt-8">
		{#if email}
			<div class="flex flex-col gap-1 font-light">
				<form method="POST" use:resendEnhance>
					<p class="text-muted-foreground text-base">
						{m.resend_otp()}
						<button
							type="submit"
							class="text-primary inline-flex items-center gap-2 underline"
						>
							{m.send_again()}
							{#if resendLoader.loading}<Spinner />{/if}
						</button>
					</p>
				</form>
			</div>
		{/if}

		<div class="flex flex-col gap-1 font-light">
			<p class="text-muted-foreground text-base">
				{m.dont_have_an_account_signup().split('?')[0]}?
				<a
					href={resolve(`/auth/signup?backTo=${encodeURIComponent(backTo ?? '/')}`)}
					class="text-primary underline"
				>
					{m.sign_up()}
				</a>
			</p>
		</div>
	</div>
{:else}
	<div class="space-y-6 lg:space-y-8">
		<div class="flex flex-col items-center gap-3 lg:gap-6">
			<h1
				class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-13"
			>
				{m.something_went_wrong()}
			</h1>
			<Button
				href={resolve(`/auth/login-otp/send?backTo=${encodeURIComponent(backTo ?? '/')}`)}
				variant="outline"
				size="lg"
				class="h-12 w-full px-7 lg:w-auto"
			>
				{m.send_again()}
			</Button>
		</div>
	</div>
{/if}
