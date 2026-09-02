<script lang="ts">
	// PROTOTYPE - throwaway. Four variants of the cookie consent moment, switchable via
	// ?variant= on the landing page. See NOTES.md in this folder.
	import { page } from '$app/state';
	import PrototypeSwitcher from './PrototypeSwitcher.svelte';
	import VariantA from './VariantA.svelte';
	import VariantB from './VariantB.svelte';
	import VariantC from './VariantC.svelte';
	import VariantD from './VariantD.svelte';

	const variants = [
		{ key: 'A', name: 'Nothing to decide' },
		{ key: 'B', name: 'Swipe deck' },
		{ key: 'C', name: 'Ask me anything' },
		{ key: 'D', name: 'The jar' }
	];

	let current = $derived((page.url.searchParams.get('variant') ?? 'A').toUpperCase());
	// Dismissing should not end the prototype, so the switcher can put it back.
	let dismissedFor = $state<string | null>(null);
	let showing = $derived(dismissedFor !== current);

	function onDone() {
		dismissedFor = current;
	}
</script>

<!-- The real vanilla-cookieconsent modal is mounted in the root layout and blocks page
	interaction. Hide it while the prototype is on screen. -->
<svelte:head>
	<style>
		#cc-main {
			display: none !important;
		}
		html.disable--interaction,
		html.disable--interaction body {
			overflow: auto !important;
		}
	</style>
</svelte:head>

{#if showing}
	{#if current === 'A'}
		<VariantA {onDone} />
	{:else if current === 'B'}
		<VariantB {onDone} />
	{:else if current === 'C'}
		<VariantC {onDone} />
	{:else if current === 'D'}
		<VariantD {onDone} />
	{/if}
{:else}
	<p class="text-muted-foreground fixed top-16 left-1/2 z-50 -translate-x-1/2 text-base">
		Dismissed. Hit replay on the bar to see it again.
	</p>
{/if}

<PrototypeSwitcher {variants} {current} onReset={() => (dismissedFor = null)} />
