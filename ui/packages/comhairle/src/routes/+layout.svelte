<script lang="ts">
	import type { LayoutProps } from './$types';
	import CookieConsent from '$lib/cookies/cookieconsent.svelte';
	import ThemeProvider from '$lib/components/ThemeProvider.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { browser } from '$app/environment';
	import '../app.css';
	import { afterNavigate } from '$app/navigation';
	import { notifications, NotificationsToaster } from '$lib/notifications.svelte';

	let { children, data }: LayoutProps = $props();

	let { themeName, isCommunity } = data;

	// Theme name is always determined by the PUBLIC_THEME env var
	if (browser && themeName) {
		themeStore.initFromServer(themeName);
	}

	$effect(() => {
		notifications.listen();
	});

	afterNavigate(() => {
		notifications.showFlash();
	});
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<ThemeProvider>
	<div class="bg-background w-full">
		<CookieConsent />
		<NotificationsToaster closeButton />
		{@render children()}
	</div>
</ThemeProvider>
