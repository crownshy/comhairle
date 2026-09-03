<script lang="ts">
	import type { LayoutProps } from './$types';
	import CookieConsent from '$lib/cookies/cookieconsent.svelte';
	import ThemeProvider from '$lib/components/ThemeProvider.svelte';
	import RouteProgress from '$lib/components/RouteProgress.svelte';
	import { browser } from '$app/environment';
	import '../app.css';
	import { afterNavigate } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';

	let { children, data }: LayoutProps = $props();
	let { isCommunity } = data;

	import { UmamiAnalytics, status } from '@lukulent/svelte-umami';
	import { env } from '$env/dynamic/public';

	const umamiWebsiteID = env.PUBLIC_UMAMI_WEBSITE_ID;
	const umamiSrcURL = env.PUBLIC_UMAMI_SRC;
	const umamiRecorderSrcURL = env.PUBLIC_UMAMI_RECORDER_SRC;
	const umamiEnabled = $derived(browser && !!umamiWebsiteID && !!umamiSrcURL);
	const umamiRecordingEnabled = $derived(umamiEnabled && !!umamiRecorderSrcURL);

	// Send the logged-in user's id to Umami once the script has loaded, and
	// re-run whenever the user (login/logout) or script status changes.
	const userId = $derived(data.user?.id ?? null);
	$effect(() => {
		if ($status !== 'loaded') return;
		window.umami?.identify(userId ? String(userId) : null);
	});

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

	{#if umamiRecordingEnabled}
		<script async defer src={umamiRecorderSrcURL} data-website-id={umamiWebsiteID}></script>
	{/if}
</svelte:head>

{#if umamiEnabled}
	<UmamiAnalytics websiteID={umamiWebsiteID!} srcURL={umamiSrcURL!} />
{/if}

<ThemeProvider>
	<div class="bg-background w-full">
		<RouteProgress />
		<CookieConsent />
		<!-- Toasts temporarily disabled while we simplify the participant chrome. -->
		<!-- <NotificationsToaster closeButton /> -->
		{@render children()}
	</div>
</ThemeProvider>
