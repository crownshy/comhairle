<script lang="ts">
	import { apiClient } from '$lib/api/client';
	import { toast } from 'svelte-sonner';

	let { user } = $props();

	let responseMessage: string | null = $state(null);

	async function resendVerificationEmail() {
		try {
			await apiClient.ResendVerificationEmail({ user });
			toast.success('Verification email re-sent. Please check your inbox.');
		} catch (e) {
			responseMessage = 'Something went wrong re-sending verification email.';
			if (!user) {
				responseMessage +=
					'\nPlease try logging in. You will be notified at a later date to re-attempt email verification.';
			}
		}
	}
</script>

<section class="">
	<h1 class="mb-2 text-xl font-bold">We have sent you an email</h1>
	<p class="mb-4">Click on the verification link we have sent to your email address.</p>
	{#if responseMessage}
		<p class="text-destructive mb-4 text-sm whitespace-break-spaces">{responseMessage}</p>
	{/if}
	<p class="text-sm">
		Didn&apos;t receive the email yet? Check your spam folder or
		<button type="button" class="underline" onclick={resendVerificationEmail}> Send again. </button>
	</p>
</section>
