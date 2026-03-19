<script lang="ts">
	import { apiClient } from '@crownshy/api-client/client';
	import { toast } from 'svelte-sonner';
	import * as m from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';

	let { user, backTo }: { user: any; backTo?: string } = $props();

	let responseMessage: string | null = $state(null);

	async function resendVerificationEmail() {
		try {
			await apiClient.ResendVerificationEmail({ id: user.id });
			toast.success(m.verification_email_sent());
		} catch (e) {
			responseMessage = m.verify_error_message();
			if (!user) {
				responseMessage += m.verify_missing_user();
			}
		}
	}
</script>

<div class="dark:bg-card mx-4 w-full max-w-lg rounded-xl bg-white p-6 shadow-sm lg:mx-0 lg:p-10">
	<div class="flex flex-col items-center gap-3 lg:gap-6">
		<h1
			class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-[52px]"
		>
			{m.verification_message_heading()}
		</h1>
		<p
			class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
		>
			{m.verification_message_body()}
		</p>

		{#if responseMessage}
			<p class="text-destructive text-center text-sm whitespace-break-spaces">
				{responseMessage}
			</p>
		{/if}

		<p class="text-muted-foreground text-center text-sm">
			{m.verification_email_missing()}
			<button
				type="button"
				class="text-primary underline underline-offset-4"
				onclick={resendVerificationEmail}
			>
				{m.send_again()}
			</button>
		</p>

		<div class="flex flex-col items-center gap-3 pt-2 sm:flex-row">
			<Button
				href={`/auth/login?backTo=${encodeURIComponent(backTo ?? '/')}`}
				variant="default"
				size="lg"
			>
				{m.back_to_login()}
			</Button>
			<Button href={backTo ?? '/'} variant="outline" size="lg">
				{m.go_home()}
			</Button>
		</div>
	</div>
</div>
