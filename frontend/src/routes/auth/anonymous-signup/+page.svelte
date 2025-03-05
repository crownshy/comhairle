<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
</script>

<AuthPage>
	<form class="space-y-4" method="POST">
		<h1 class="text-xl">{m.sign_up_anonymously()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{m.get_started_with_comhairle_right_away()}</p>
		<Form.Button fullWidth variant="default">{m.generate_pseudonymous_id()} →</Form.Button>
		<p class="text-sm"><a href="/auth/login">{m.already_have_an_account_login()}</a></p>
	</form>
</AuthPage>
