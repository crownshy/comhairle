<script lang="ts">
	import type { LayoutProps } from './$types';
	import CookieConsent from '$lib/cookies/cookieconsent.svelte';
	import '../app.css';
	import { afterNavigate } from '$app/navigation';
	import { notifications, NotificationsToaster } from '$lib/notifications.svelte';
	import ThemeProvider from '$lib/components/ThemeProvider.svelte';

	let { children }: LayoutProps = $props();

	$effect(() => {
		notifications.listen();
	});

	afterNavigate(() => {
		notifications.showFlash();
	});
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<ThemeProvider defaultTheme="comhairle">
	<div class="w-full bg-background">
		<CookieConsent />
		<NotificationsToaster closeButton />
		{@render children()}
	</div>
</ThemeProvider>
