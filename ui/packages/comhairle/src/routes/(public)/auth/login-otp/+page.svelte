<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginOtpForm from './login-otp-form.svelte';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Log in - Comhairle</title>
</svelte:head>

<AuthLayout>
	<LoginOtpForm backTo={data.backTo} />
</AuthLayout>
