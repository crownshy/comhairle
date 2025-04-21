<script lang="ts">
	import type { LayoutProps } from './$types';
	import { i18n } from '$lib/i18n';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
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
</script>

<CookieConsent />
<ParaglideJS {i18n}>
	<NotificationsToaster closeButton />
	{@render children()}
</ParaglideJS>
