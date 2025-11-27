<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import Label from '$lib/components/ui/label/label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
</script>

<AuthPage>
	<section>
		<form class="space-y-4">
			<h1 class="text-xl font-bold">{m.create_new_password_heading()}</h1>
			<Label>{m.new_password()}</Label>
			<Input />
			<!-- TODO: use PasswordInput from other branch -->
			<Label>{m.confirm_password()}</Label>
			<Input />
			<!-- TODO: use PasswordInput from other branch -->
			<Button type="button">{m.submit()}</Button>
		</form>
	</section>
</AuthPage>
