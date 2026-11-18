<script lang="ts">
	import { untrack } from 'svelte';
	import { page } from '$app/state';
	import Footer from '$lib/components/Footer.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { DEFAULT_THEME } from '$lib/types/theme';
	import LabTopBar from './components/LabTopBar.svelte';
	import ScenarioSwitcher from './components/ScenarioSwitcher.svelte';
	import { SAMPLE_USER } from './mockData';
	import { readScenario } from './scenario';
	import { COMHAIRLE_VARS } from './baseTheme';

	let { children } = $props();

	const scenario = $derived(readScenario(page.url));
	const theme = $derived(scenario.org.theme);

	// Until the browser has swapped <html data-theme>, Comhairle needs its colours inline, or
	// the server render shows the app's own theme first (see baseTheme.ts). Dropped after that
	// so dark mode, which the inline light colours would block, works normally.
	let themeApplied = $state(false);
	const wrapperStyle = $derived(
		theme === DEFAULT_THEME && !themeApplied ? COMHAIRLE_VARS : undefined
	);

	// The real app takes its theme from PUBLIC_THEME. The prototype swaps it per organisation:
	// the data-theme attribute on <html> brings in that theme's colours, and the theme store
	// brings in its logo (the footer reads it). Both go back when you leave the prototype.
	$effect(() => {
		const root = document.documentElement;
		const previousAttr = root.getAttribute('data-theme');
		const previousName = untrack(() => themeStore.name);
		if (theme === DEFAULT_THEME) root.removeAttribute('data-theme');
		else root.setAttribute('data-theme', theme);
		themeStore.initFromServer(theme);
		themeApplied = true;
		return () => {
			if (previousAttr) root.setAttribute('data-theme', previousAttr);
			else root.removeAttribute('data-theme');
			themeStore.initFromServer(previousName);
		};
	});
</script>

<svelte:head>
	<meta name="robots" content="noindex" />
</svelte:head>

<!-- The page sits on a faint tint of the theme's primary colour so white cards stand out.
	data-theme (or the inline Comhairle colours) here too, so the server render is already in
	the chosen theme, before the effect runs. -->
<div
	data-theme={theme === DEFAULT_THEME ? undefined : theme}
	style={wrapperStyle}
	class="flex min-h-screen w-full flex-col overflow-x-clip bg-[var(--page-tint)] [--page-tint:color-mix(in_oklab,var(--primary)_6%,var(--background))]"
>
	<LabTopBar
		org={scenario.org}
		loggedIn={scenario.loggedIn}
		userInitials={SAMPLE_USER.initials}
		overHero={!scenario.loggedIn && page.route.id === '/(lab)/home-lab'}
	/>
	<main class="mx-auto w-full max-w-[1300px] grow px-4 pb-24 md:px-8">
		{@render children()}
	</main>
	<Footer />
	<ScenarioSwitcher />
</div>
