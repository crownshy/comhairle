<script lang="ts">
	import type { LayoutProps } from './$types';
	import CookieConsent from '$lib/cookies/cookieconsent.svelte';
	import '../app.css';
	import { afterNavigate } from '$app/navigation';
	import { notifications, NotificationsToaster } from '$lib/notifications.svelte';

	let { children }: LayoutProps = $props();

	$effect(() => {
		notifications.listen();
	});

	afterNavigate(() => {
		notifications.showFlash();
	});

	let theme = {
		background: 'hsla(0, 0%, 100%, 1)',
		foreground: 'hsl(30 10% 3.9%)',
		primary: 'hsl(88, 33%, 9%)',
		secondary: 'hsl(102, 34%, 30%)',
		radius: '6.25rem',
		'secondary-foreground': 'white',
		'muted-foreground': 'hsla(79, 100%, 96%, 1)',
		'nav-background': 'hsla(78, 60%, 89%, 0.08)',
		'nav-text': 'hsla(79, 100%, 96%, 1)',
		'font-serif': 'inter',
		'font-sans': 'inter',
		'font-mono': 'inter',
		sidebar: 'hsl(88, 33%, 9%)',
		'sidebar-radius': '10px',
		'sidebar-foreground': 'hsla(79, 100%, 96%, 1)',
		'admin-background': 'hsla(60, 67%, 98%, 1)',
		mutted: 'hsla(100, 29%, 10%, 1)'
	};

	let themeCss = Object.entries(theme).reduce((a, b) => (a = a + `--${b[0]} : ${b[1]};`), '');
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div style={themeCss} class="w-full bg-stone-50">
	<CookieConsent />
	<NotificationsToaster closeButton />
	{@render children()}
</div>
