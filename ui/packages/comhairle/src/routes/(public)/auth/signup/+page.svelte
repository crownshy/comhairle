<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import { AuthPage } from '$lib/profile';
	let { data }: PageData = $props();
	let backTo = $derived(data.backTo);
</script>

<svelte:head>
	<title>Sign Up - Comhairle</title>
</svelte:head>

<AuthPage>
	<SignupForm {backTo} />
</AuthPage>
