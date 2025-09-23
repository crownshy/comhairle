<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import { page } from '$app/state';

	let { data } = $props();
	let backTo = page.url.searchParams.get('backTo') ?? '/';
</script>

<AuthPage>
	<div class="space-y-4">
		<p class="text-muted-foreground mb-4 text-sm">{m.get_started_with_comhairle_right_away()}</p>

		<h3 class="text-center">Your Id is</h3>
		<CopyButton copyText={data?.user?.username}>
			<h2 class="text-center text-2xl font-bold">
				{data?.user?.username}
			</h2>
		</CopyButton>
		<p class="text-center">Write this down somewhere safe and use it to log back in.</p>

		<p class="text-muted-foreground mb-4 text-sm">{m.keey_this_pseudonymous_id_safe()}</p>

		<Button href={backTo} variant="secondary" class="w-full">{m.continue_()}</Button>
		<p class="text-sm">
			<a href={`/auth/signup?backTo=${backTo}`}>{m.create_a_standard_account_signup()}</a>
		</p>
	</div>
</AuthPage>
