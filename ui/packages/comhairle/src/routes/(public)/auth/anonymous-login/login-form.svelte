<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { annonLoginFormSchema } from '$lib/profile';
	import { type SuperValidated, superForm, defaults } from 'sveltekit-superforms';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';

	let { backTo }: { backTo?: string } = $props();

	const form = superForm(defaults(zod(annonLoginFormSchema)), {
		validators: zodClient(annonLoginFormSchema),
		onSubmit: attemptLogin
	});

	const { form: formData, enhance, message: errMessage, validateForm } = form;
	const loader = useLoading();

	async function attemptLogin() {
		let result = await validateForm({ update: true });
		if (result.valid) {
			let { username } = result.data;
			await loader.run(async () => {
				try {
					await apiClient.LoginAnnonUser({
						username
					});
					await invalidateAll();
					await goto(backTo ?? '/');
				} catch (e) {
					$errMessage = e.response.data.err;
				}
			});
		}
	}
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.login_with_anonymous_id()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{m.enter_your_details_below_to_login()}</p>
	</div>
	{#if $errMessage}
		<p class="text-destructive text-sm">{$errMessage}</p>
	{/if}
	<Form.Field {form} name="username">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>{m.id()}</Form.Label>
				<Input {...props} bind:value={$formData.username} required />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<LoadingButton type="submit" class="w-full" variant="secondary" loading={loader.loading}>
		{m.submit()}
	</LoadingButton>
	<p class="text-sm">
		<a href={`/auth/signup?backTo=${backTo ?? '/'}`}>{m.dont_have_an_account_signup()}</a>
	</p>
</form>
