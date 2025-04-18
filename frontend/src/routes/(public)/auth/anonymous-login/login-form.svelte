<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { annonLoginFormSchema } from '$lib/profile';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';

	let {
		data,
		backTo
	}: { data: SuperValidated<Infer<typeof annonLoginFormSchema>>; backTo?: string } = $props();

	console.log('DATA IS ', data);
	const form = superForm(data, {
		validators: zodClient(annonLoginFormSchema)
	});

	const { form: formData, enhance, message: errMessage } = form;
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.login_with_pseudonymous_id()}</h1>
		<p class="mb-4 text-sm text-muted-foreground">{m.enter_your_details_below_to_login()}</p>
	</div>
	{#if $errMessage}
		<p class="text-sm text-destructive">{$errMessage}</p>
	{/if}
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Form.Label>{m.id()}</Form.Label>
			<Input {...attrs} bind:value={$formData.username} required />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button fullWidth variant="default">{m.submit()}</Form.Button>
	<p class="text-sm">
		<a href={`/auth/signup?backTo=${backTo ?? '/'}`}>{m.dont_have_an_account_signup()}</a>
	</p>
</form>
