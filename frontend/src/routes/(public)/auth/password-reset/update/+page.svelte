<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import Label from '$lib/components/ui/label/label.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import PasswordInput from '$lib/components/ui/password-input/password-input.svelte';
</script>

<AuthPage>
	<section>
		<form class="space-y-4">
			<h1 class="text-xl font-bold">{m.create_new_password_heading()}</h1>
			<Label>{m.new_password()}</Label>
			<PasswordInput placeholder={m.new_password()} />
			<Label>{m.confirm_password()}</Label>
			<PasswordInput placeholder={m.confirm_password()} />
			<Button type="button">{m.submit()}</Button>
		</form>
	</section>
</AuthPage>
