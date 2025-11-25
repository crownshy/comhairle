<script lang="ts">
	import { apiClient } from '$lib/api/client';
	import { onMount } from 'svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as m from '$lib/paraglide/messages';

	let { token } = $props();

	let status: 'idle' | 'loading' | 'success' | 'error' = $state('idle');
	let errorMessage = $state(
		m.verify_user_error_body(),
	);

	async function verifyEmailToken() {
		status = 'loading';
		try {
			await apiClient.VerifyEmailToken({ token });
			status = 'success';
		} catch (e) {
			if (e.response.status === 409) {
				errorMessage = m.verify_user_conflict_error_body();
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
		<h1 class="mb-2 text-xl font-bold">{m.verify_user_pending_heading()}</h1>
		{#if status === 'loading'}
			<div class="flex justify-center">
				<Spinner />
			</div>
		{:else}
			<p class="mb-4">{m.verify_user_pending_body()}</p>
		{/if}
	{:else if status === 'success'}
		<h1 class="mb-2 text-xl font-bold">{m.verify_user_success_heading()}</h1>
		<p class="mb-4">{@html m.verify_user_success_body()}</p>
	{:else if status === 'error'}
		<h1 class="mb-2 text-xl font-bold">{m.verify_user_error_heading()}</h1>
		<p class="mb-4">
			{@html errorMessage}
		</p>
	{/if}
</section>
