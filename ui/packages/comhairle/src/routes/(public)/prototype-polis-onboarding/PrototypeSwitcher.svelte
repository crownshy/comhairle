<script lang="ts">
	// PROTOTYPE - throwaway. Floating variant switcher plus an audience toggle, hidden
	// outside dev. Copied from prototype-cookie-consent so either folder can be deleted alone.
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import { ChevronLeft, ChevronRight, RotateCcw } from 'lucide-svelte';
	import type { Audience } from './practice';

	let {
		variants,
		current,
		audience,
		onReset
	}: {
		variants: { key: string; name: string }[];
		current: string;
		audience: Audience;
		onReset: () => void;
	} = $props();

	let index = $derived(
		Math.max(
			0,
			variants.findIndex((v) => v.key === current)
		)
	);
	let label = $derived(variants[index]?.name ?? '');

	function setParam(key: string, value: string) {
		const url = new URL(page.url);
		url.searchParams.set(key, value);
		goto(url, { replaceState: true, noScroll: true, keepFocus: true });
	}

	function go(step: number) {
		setParam('variant', variants[(index + step + variants.length) % variants.length].key);
	}

	function onkeydown(event: KeyboardEvent) {
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, [contenteditable]')) return;
		if (event.key === 'ArrowLeft') go(-1);
		if (event.key === 'ArrowRight') go(1);
	}
</script>

<svelte:window {onkeydown} />

{#if dev}
	<div
		class="fixed bottom-3 left-1/2 z-[100] flex -translate-x-1/2 items-center gap-0.5 rounded-full bg-black px-1 py-1 text-base font-medium text-white shadow-xl"
	>
		<button
			type="button"
			class="inline-flex size-8 items-center justify-center rounded-full hover:bg-white/15"
			aria-label="Previous variant"
			onclick={() => go(-1)}
		>
			<ChevronLeft class="size-5" />
		</button>
		<span class="px-2 text-center whitespace-nowrap">{current} &middot; {label}</span>
		<button
			type="button"
			class="inline-flex size-8 items-center justify-center rounded-full hover:bg-white/15"
			aria-label="Next variant"
			onclick={() => go(1)}
		>
			<ChevronRight class="size-5" />
		</button>
		<button
			type="button"
			class="rounded-full px-2 py-1 hover:bg-white/15"
			aria-label="Switch audience"
			onclick={() => setParam('audience', audience === 'young' ? 'policy' : 'young')}
		>
			{audience === 'young' ? 'Young' : 'Policy'}
		</button>
		<button
			type="button"
			class="inline-flex size-8 items-center justify-center rounded-full hover:bg-white/15"
			aria-label="Replay this variant"
			onclick={onReset}
		>
			<RotateCcw class="size-4" />
		</button>
	</div>
{/if}
