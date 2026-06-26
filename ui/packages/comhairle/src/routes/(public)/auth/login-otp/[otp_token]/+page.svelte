<script lang="ts">
	import type { PageData } from './$types.js';
	import AuthLayout from '$lib/components/AuthLayout.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { resolve } from '$app/paths';

	let { data }: { data: PageData } = $props();
	const { backTo, jwt } = data;

	let error = $state<string | null>(null);

	async function attemptOtpLogin() {
		try {
			await apiClient.LoginOtpToken({ token: jwt });

			await goto(resolve(backTo), { invalidateAll: true });
		} catch (e) {
			console.error(e);
			error = 'Failed to verify one-time-passcode';
		}
	}

	onMount(() => {
		attemptOtpLogin();
	});
</script>

<svelte:head>
	<title>Send one time passcode - Comhairle</title>
</svelte:head>

<AuthLayout>
	{#if !error}
		<div class="flex h-full w-full justify-center">
			<Spinner class="h-20 w-20" />
		</div>
	{:else}
		<div class="flex flex-col gap-4">
			<h1 class="text-4xl font-semibold">Unable to verify user</h1>
			<p class="text-lg">{error}</p>
			<Button href={`/auth/login-otp/send?backTo=${encodeURIComponent(backTo)}`}
				>Resend link?</Button
			>
		</div>
	{/if}
</AuthLayout>
