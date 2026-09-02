<script lang="ts">
	// PROTOTYPE - throwaway. Floating variant switcher, hidden outside dev.
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import { ChevronLeft, ChevronRight, RotateCcw } from 'lucide-svelte';

	let {
		variants,
		current,
		onReset
	}: {
		variants: { key: string; name: string }[];
		current: string;
		onReset: () => void;
	} = $props();

	let index = $derived(
		Math.max(
			0,
			variants.findIndex((v) => v.key === current)
		)
	);
	let label = $derived(variants[index]?.name ?? '');

	function go(step: number) {
		const next = variants[(index + step + variants.length) % variants.length];
		const url = new URL(page.url);
		url.searchParams.set('variant', next.key);
		goto(url, { replaceState: true, noScroll: true, keepFocus: true });
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
		class="fixed top-2 right-2 z-[100] flex items-center gap-0.5 rounded-full bg-black px-1 py-1 text-base font-medium text-white shadow-xl"
	>
		<button
			type="button"
			class="inline-flex size-8 items-center justify-center rounded-full hover:bg-white/15"
			aria-label="Previous variant"
			onclick={() => go(-1)}
		>
			<ChevronLeft class="size-5" />
		</button>
		<span class="px-2 text-center whitespace-nowrap">
			{current} &middot; {label}
		</span>
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
			class="inline-flex size-8 items-center justify-center rounded-full hover:bg-white/15"
			aria-label="Replay this variant"
			onclick={onReset}
		>
			<RotateCcw class="size-4" />
		</button>
	</div>
{/if}
