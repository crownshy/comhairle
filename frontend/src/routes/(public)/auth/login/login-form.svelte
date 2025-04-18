<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { loginFormSchema } from '$lib/profile';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as m from '$lib/paraglide/messages';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';

	let { data, backTo }: { data: SuperValidated<Infer<typeof loginFormSchema>>; backTo?: string } =
		$props();

	const form = superForm(data, {
		validators: zodClient(loginFormSchema)
	});

	const { form: formData, enhance, message: errMessage } = form;
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.login()}</h1>
		<p class="mb-4 text-sm text-muted-foreground">{m.enter_your_details_below_to_login()}</p>
	</div>
	{#if $errMessage}
		<p class="text-sm text-destructive">{$errMessage}</p>
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
	<a
		href={`/auth/anonymous-login?backTo=${backTo ?? '/'}`}
		class={cn('w-full', buttonVariants({ variant: 'outline' }))}
	>
		{m.Login_with_Pseudonymous_ID()}
	</a>
	<p class="text-sm">
		<a href={`/auth/signup?backTo=${backTo ?? '/'}`}>{m.dont_have_an_account_signup()}</a>
	</p>
</form>
