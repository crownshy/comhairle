<script lang="ts">
	import type { PageData } from './$types.js';
	import EmailOtpForm from './email-otp-form.svelte';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Send one time passcode - Comhairle</title>
</svelte:head>

<AuthLayout>
	<EmailOtpForm backTo={data.backTo} />
</AuthLayout>
