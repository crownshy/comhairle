<script lang="ts">
	// PROTOTYPE - throwaway. Four onboarding variants for people who have never used Polis
	// (#1203), switchable via ?variant=A..D and ?audience=young|policy. Each one hands off to
	// a stand-in for the real voting screen. See NOTES.md in this folder.
	import { page } from '$app/state';
	import { X } from 'lucide-svelte';
	import PrototypeSwitcher from './PrototypeSwitcher.svelte';
	import FakeVoting from './FakeVoting.svelte';
	import VariantA from './VariantA.svelte';
	import VariantB from './VariantB.svelte';
	import VariantC from './VariantC.svelte';
	import VariantD from './VariantD.svelte';
	import { COPY, type Audience } from './practice';

	const variants = [
		{ key: 'A', name: 'Poll reveal' },
		{ key: 'B', name: 'Coach marks' },
		{ key: 'C', name: 'Living map' },
		{ key: 'D', name: 'Myth or fact' }
	];

	let current = $derived((page.url.searchParams.get('variant') ?? 'A').toUpperCase());
	let audience = $derived<Audience>(
		page.url.searchParams.get('audience') === 'policy' ? 'policy' : 'young'
	);

	let run = $state(0);
	let finishedRun = $state<string | null>(null);
	let runKey = $derived(`${current}-${audience}-${run}`);
	let onboarding = $derived(finishedRun !== runKey);

	let realIndex = $state(0);

	function onDone() {
		finishedRun = runKey;
		realIndex = 0;
	}
</script>

<!-- The cookie modal from the root layout would sit on top of every variant. -->
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

<div class="bg-background mx-auto flex h-dvh w-full max-w-[640px] flex-col gap-4 px-4 pt-3 pb-16">
	<!-- Rough stand-in for StepChrome, so the onboarding is judged inside a step and not on
	     an empty page. -->
	<header class="flex items-center justify-between">
		<div class="flex flex-col">
			<span class="text-muted-foreground text-base">Step 2 of 4</span>
			<span class="text-foreground text-lg font-semibold">Share your views</span>
		</div>
		<X class="text-muted-foreground size-6" />
	</header>

	{#key runKey}
		{#if onboarding}
			{#if current === 'A'}
				<VariantA {audience} {onDone} />
			{:else if current === 'B'}
				<VariantB {audience} {onDone} />
			{:else if current === 'C'}
				<VariantC {audience} {onDone} />
			{:else if current === 'D'}
				<VariantD {audience} {onDone} />
			{/if}
		{:else}
			<FakeVoting
				statement={COPY[audience].real[realIndex % COPY[audience].real.length]}
				counter="Opinion {(realIndex % 24) + 1} of 24"
				onVote={() => realIndex++}
			/>
		{/if}
	{/key}
</div>

<PrototypeSwitcher {variants} {current} {audience} onReset={() => run++} />
