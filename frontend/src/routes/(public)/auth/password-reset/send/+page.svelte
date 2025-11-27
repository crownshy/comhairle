<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
</script>

<AuthPage>
	<section>
		<div class="mb-4 space-y-4">
			<h1 class="text-xl font-bold">{m.reset_password_heading()}</h1>
			<p class="text-muted-foreground mb-4 text-sm">{m.reset_password_body()}</p>
		</div>
		<form class="space-y-4" method="POST">
			<!-- TODO: enhance -->
			<Label>{m.email()}</Label>
			<Input placeholder={m.email_placeholder()} />
			<Button type="submit">{m.submit()}</Button>
		</form>
	</section>
</AuthPage>
