<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import type { PageData } from './$types';

	let { data }: PageData = $props();
</script>

<AuthPage>
	<form class="space-y-4" method="POST">
		<h1 class="text-xl">{m.sign_up_anonymously()}</h1>
		<p class="text-foreground mb-4 text-sm">{m.get_started_with_comhairle_right_away()}</p>
		<Form.Button class="w-full" variant="secondary">{m.generate_anonymous_id()} →</Form.Button>

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
