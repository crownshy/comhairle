<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { loginFormSchema } from '$lib/profile';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';

	export let data: SuperValidated<Infer<typeof loginFormSchema>>;

	const form = superForm(data, {
		validators: zodClient(loginFormSchema)
	});

	const { form: formData, enhance, message: errMessage } = form;
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.login()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{m.enter_your_details_below_to_login()}</p>
	</div>
	{#if $errMessage}
		<p class="text-destructive text-sm">{$errMessage}</p>
	{/if}
	<Form.Field {form} name="email">
		<Form.Control let:attrs>
			<Form.Label>{m.email()}</Form.Label>
			<Input {...attrs} bind:value={$formData.email} required />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Form.Label>{m.password()}</Form.Label>
			<Input type="password" {...attrs} bind:value={$formData.password} required />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button fullWidth variant="default">{m.submit()}</Form.Button>
	<p class="text-sm"><a href="/auth/signup">{m.dont_have_an_account_signup()}</a></p>
</form>
