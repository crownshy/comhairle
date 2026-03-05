<script lang="ts">
	import { enhance } from '$app/forms';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import type { PageData } from './$types';
	import { LoadingButton } from '$lib/components/ui/button';

	let { data }: PageData = $props();
	let loading = $state(false);
</script>

<svelte:head>
	<title>Anonymous Sign Up - Comhairle</title>
</svelte:head>

<AuthPage>
	<form
		class="space-y-4"
		method="POST"
		use:enhance={() => {
			loading = true;
			return async ({ update }) => {
				await update();
				loading = false;
			};
		}}
	>
		<h1 class="text-xl">{m.sign_up_anonymously()}</h1>
		<p class="text-foreground mb-4 text-sm">{m.get_started_with_comhairle_right_away()}</p>
		<LoadingButton type="submit" class="w-full" variant="secondary" loading={loading}>
			{m.generate_anonymous_id()} →
		</LoadingButton>

		<p class="text-muted-foreground mb-4 text-sm">
			{m.agree_to_tos()}
			<a href="/rights/tos">TOS</a>
			{m.agree_to_tos2()}
			<a href="/rights/privacy">{m.agree_to_tos_privacy()}</a>
		</p>
		<p class="text-sm">
			<a href={`/auth/login?backTo=${data.backTo ?? '/'}`}>{m.already_have_an_account_login()}</a>
		</p>
	</form>
</AuthPage>
