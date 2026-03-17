<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import AuthLayout from '$lib/components/AuthLayout.svelte';

	let { data }: { data: PageData } = $props();
	let backTo = $derived(data.backTo);
</script>

<svelte:head>
	<title>Sign Up - Comhairle</title>
</svelte:head>

<AuthLayout>
	<SignupForm {backTo} />
</AuthLayout>
