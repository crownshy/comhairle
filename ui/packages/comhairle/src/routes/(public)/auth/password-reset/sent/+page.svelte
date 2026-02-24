<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import * as m from '$lib/paraglide/messages';
	import { MailCheck } from 'lucide-svelte';
</script>

<AuthPage>
	<section class="space-y-4">
		<div class="flex justify-center">
			<MailCheck class="h-12 w-12" />
		</div>
		<h1 class="text-xl font-bold">{m.password_reset_instructions_heading()}</h1>
		<p class="text-muted-foreground text-sm">{m.password_reset_instructions_body()}</p>
	</section>
</AuthPage>
