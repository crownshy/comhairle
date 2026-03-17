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

<form class="space-y-6 lg:space-y-8" method="POST" use:enhance>
	<div class="flex flex-col items-center gap-3 lg:gap-6">
		<h1
			class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-[52px]"
		>
			{m.login_with_anonymous_id()}
		</h1>
		<p
			class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
		>
			{m.good_to_see_you_back()}
		</p>
	</div>

	{#if $errMessage}
		<p class="text-destructive text-center text-sm">{$errMessage}</p>
	{/if}

	<div class="space-y-6">
		<Form.Field {form} name="username">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>{m.anonymous_id()}</Form.Label>
					<Input
						{...props}
						placeholder={m.anonymous_id()}
						bind:value={$formData.username}
						required
					/>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
	</div>

	<div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:gap-4">
		<LoadingButton
			type="submit"
			size="lg"
			class="h-12 w-full px-7 lg:w-auto"
			variant="default"
			loading={loader.loading}
		>
			{m.submit()}
		</LoadingButton>
	</div>

	<div class="flex flex-col gap-1">
		<p class="text-muted-foreground text-base">
			{m.dont_have_an_account_signup().split('?')[0]}?
			<a href={`/auth/signup?backTo=${backTo ?? '/'}`} class="text-primary underline"
				>{m.sign_up()}</a
			>
		</p>
	</div>
</form>
