<script lang="ts">
	import { apiClient } from '$lib/api/client';
	import { onMount } from 'svelte';
	import { Spinner } from '$lib/components/ui/spinner';

	let { token } = $props();

	let status: 'idle' | 'loading' | 'success' | 'error' = $state('idle');
	let errorMessage = $state(
		'Something went wrong verifying your email address. Try <a href="/auth/login" class="underline">logging in.</a>'
	);

	async function verifyEmailToken() {
		status = 'loading';
		try {
			await apiClient.VerifyEmailToken({ token });
			status = 'success';
		} catch (e) {
			if (e.response.status === 409) {
				errorMessage =
					'Your email address has already been verified. You can now view and <a href="/conversations" class="underline">join conversations.</a>';
			}
			status = 'error';
		}
	}

	onMount(() => {
		verifyEmailToken();
	});
</script>

<section class="">
	{#if status === 'loading' || status === 'idle'}
		<h1 class="mb-2 text-xl font-bold">Verifying your email</h1>
		{#if status === 'loading'}
			<div class="flex justify-center">
				<Spinner />
			</div>
		{:else}
			<p class="mb-4">Click on the verification link we have sent to your email address.</p>
		{/if}
	{:else if status === 'success'}
		<h1 class="mb-2 text-xl font-bold">Successfully verified your email</h1>
		<p class="mb-4">
			Please <a href="/auth/login" class="underline">log in</a> to your verified account to join a conversation.
		</p>
	{:else if status === 'error'}
		<h1 class="mb-2 text-xl font-bold">Something went wrong</h1>
		<p class="mb-4">
			{@html errorMessage}
		</p>
	{/if}
</section>
