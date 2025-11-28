<script lang="ts">
	import AuthPage from '$lib/profile/AuthPage.svelte';
	import * as m from '$lib/paraglide/messages';
</script>

<AuthPage>
	<section class="space-y-4">
		<h1 class="text-xl font-bold">{m.password_reset_success_heading()}</h1>
		<p class="text-muted-foreground mb-4 text-sm">{@html m.password_reset_success_body()}</p>
	</section>
</AuthPage>
