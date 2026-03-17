<script lang="ts">
	import type { PageData } from './$types.js';
	import LoginForm from './login-form.svelte';
	import AuthGradient from '$lib/components/AuthGradient.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Log in - Comhairle</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col lg:flex-row">
	<AuthGradient />

	<div
		class="bg-background flex w-full flex-1 items-center justify-center px-5 py-10 lg:w-1/2 lg:px-6 lg:py-0"
	>
		<div class="w-full max-w-md">
			<LoginForm backTo={data.backTo} />
		</div>
	</div>
</div>
