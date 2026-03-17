<script lang="ts">
	import type { PageData } from './$types.js';
	import SignupForm from './signup-form.svelte';
	import AuthGradient from '$lib/components/AuthGradient.svelte';

	let { data }: { data: PageData } = $props();
	let backTo = $derived(data.backTo);
</script>

<svelte:head>
	<title>Sign Up - Comhairle</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col lg:flex-row">
	<div
		class="bg-background order-2 flex w-full flex-1 items-center justify-center px-5 py-10 lg:order-1 lg:w-1/2 lg:px-6 lg:py-0"
	>
		<div class="w-full max-w-md">
			<SignupForm {backTo} />
		</div>
	</div>

	<div class="order-1 lg:order-2 lg:contents">
		<AuthGradient />
	</div>
</div>
