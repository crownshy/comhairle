<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import { page } from '$app/state';

	let { data } = $props();
	let backTo = page.url.searchParams.get('backTo') ?? '/';
</script>

<AuthPage>
	<div class="space-y-4">
		<h1 class="text-xl">{m.sign_up_anonymously()}</h1>
		<p class="mb-4 text-sm text-muted-foreground">{m.get_started_with_comhairle_right_away()}</p>

		<a href={backTo} class={cn('w-full', buttonVariants({ variant: 'default' }))}>{m.continue_()}</a
		>

		<Input disabled value={data?.user?.username} />

		<p class="mb-4 text-sm text-muted-foreground">{m.keey_this_pseudonymous_id_safe()}</p>

		<p class="text-sm">
			<a href={`/auth/signup?backTo=${backTo}`}>{m.create_a_standard_account_signup()}</a>
		</p>
	</div>
</AuthPage>
