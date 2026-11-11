<script lang="ts">
	import { page } from '$app/state';
	import Footer from '$lib/components/Footer.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import LabTopBar from './components/LabTopBar.svelte';
	import ScenarioSwitcher from './components/ScenarioSwitcher.svelte';
	import { SAMPLE_USER } from './mockData';
	import { brandStyle, orgDisplayName, readScenario } from './scenario';

	let { children } = $props();

	const scenario = $derived(readScenario(page.url));
	const PAGE_TINT = '--page-tint: color-mix(in oklab, var(--primary) 6%, var(--background));';
	const pageStyle = $derived(`${brandStyle(scenario.org)} ${PAGE_TINT}`);
</script>

<svelte:head>
	<meta name="robots" content="noindex" />
</svelte:head>

<!-- Overriding --primary here re-colours every staging component inside with the org brand.
	The page sits on a faint tint of that colour so white cards stand out from it. -->
<div
	class="flex min-h-screen w-full flex-col overflow-x-clip bg-[var(--page-tint)]"
	style={pageStyle}
>
	<LabTopBar
		org={scenario.org}
		orgName={orgDisplayName(scenario.org, themeStore.name)}
		loggedIn={scenario.loggedIn}
		userInitials={SAMPLE_USER.initials}
	/>
	<main class="mx-auto w-full max-w-[1300px] grow px-4 pb-24 md:px-8">
		{@render children()}
	</main>
	<Footer />
</div>
<ScenarioSwitcher />
