<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { signupFormSchema } from '$lib/profile';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';

	let { data, backTo }: { data: SuperValidated<Infer<typeof signupFormSchema>>; backTo?: string } =
		$props();

	const form = superForm(data, {
		validators: zodClient(signupFormSchema)
	});

	const { form: formData, enhance, message: errMessage } = form;
</script>

<form class="space-y-4" method="POST" use:enhance>
	<div>
		<h1 class="text-xl font-bold">{m.create_an_account()}</h1>
		<p class="mb-4 text-sm text-muted-foreground">{m.get_started_with_comhairle_today()}</p>
	</div>
	{#if $errMessage}
		<p class="text-sm text-destructive">{$errMessage}</p>
	{/if}
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Form.Label>{m.username()}</Form.Label>
			<Input {...attrs} bind:value={$formData.username} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="email">
		<Form.Control let:attrs>
			<Form.Label>{m.email()}</Form.Label>
			<Input {...attrs} bind:value={$formData.email} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Form.Label>{m.new_password()}</Form.Label>
			<Input type="password" {...attrs} bind:value={$formData.password} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password_confirm">
		<Form.Control let:attrs>
			<Form.Label>{m.confirm_password()}</Form.Label>
			<Input type="password" {...attrs} bind:value={$formData.password_confirm} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button fullWidth variant="default">Submit</Form.Button>
	<a
		href={`/auth/anonymous-signup?backTo=${backTo}`}
		class={cn('w-full', buttonVariants({ variant: 'outline' }))}>{m.sign_up_anonymously()}</a
	>
	<p class="text-sm">
		<a href={`/auth/login?backTo=${backTo}`}>{m.already_have_an_account_login()}</a>
	</p>
</form>
