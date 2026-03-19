<script lang="ts">
	import type { PageData } from './$types.js';
	import AuthGradient from '$lib/components/AuthGradient.svelte';
	import VerificationMessage from './verification-message.svelte';

	export let data: PageData;
</script>

<svelte:head>
	<title>Verify Your Email - Comhairle</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col">
	<AuthGradient showLogo={false} mode="full">
		<VerificationMessage user={data.user} backTo={data.backTo} />
	</AuthGradient>
</div>
