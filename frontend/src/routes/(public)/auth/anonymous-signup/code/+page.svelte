<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';

	let { data } = $props();
</script>

<AuthPage>
	<div class="space-y-4">
		<h1 class="text-xl">{m.sign_up_anonymously()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{m.get_started_with_comhairle_right_away()}</p>

		<a href="/" class={cn('w-full', buttonVariants({ variant: 'default' }))}>{m.continue_()}</a>

		<Input disabled value={data?.user?.username} />

		<p class="text-muted-foreground mb-4 text-sm">{m.keey_this_pseudonymous_id_safe()}</p>

		<p class="text-sm"><a href="/auth/signup">{m.create_a_standard_account_signup()}</a></p>
	</div>
</AuthPage>
