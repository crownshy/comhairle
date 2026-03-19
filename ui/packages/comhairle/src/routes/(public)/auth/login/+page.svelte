<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginForm from './login-form.svelte';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Log in - Comhairle</title>
</svelte:head>

<AuthLayout>
	<LoginForm backTo={data.backTo} />
</AuthLayout>
