<script lang="ts">
	import { apiClient } from '$lib/api/client';
	import { toast } from 'svelte-sonner';
	import * as m from '$lib/paraglide/messages';

	let { user } = $props();

	let responseMessage: string | null = $state(null);

	async function resendVerificationEmail() {
		try {
			await apiClient.ResendVerificationEmail({ username: user.username });
			toast.success(m.verification_email_sent());
		} catch (e) {
			responseMessage = m.verify_error_message();
			if (!user) {
				responseMessage += m.verify_missing_user();
			}
		}
	}
</script>

<section class="">
	<h1 class="mb-2 text-xl font-bold">{m.verification_message_heading()}</h1>
	<p class="mb-4">{m.verification_message_body()}</p>
	{#if responseMessage}
		<p class="text-destructive mb-4 text-sm whitespace-break-spaces">{responseMessage}</p>
	{/if}
	<p class="text-sm">
		{m.verification_email_missing()}
		<button type="button" class="underline" onclick={resendVerificationEmail}>
			{m.send_again()}.
		</button>
	</p>
</section>
