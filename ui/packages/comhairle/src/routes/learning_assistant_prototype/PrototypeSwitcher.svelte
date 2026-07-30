<!-- PROTOTYPE - throwaway. Floating variant switcher (bottom-centre) + width toggle. -->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { dev } from '$app/environment';
	import { LucideChevronLeft, LucideChevronRight } from 'lucide-svelte';

	let {
		variants,
		current,
		width
	}: {
		variants: { key: string; name: string }[];
		current: string;
		width: 'sidebar' | 'page';
	} = $props();

	let index = $derived(
		Math.max(
			0,
			variants.findIndex((v) => v.key === current)
		)
	);

	function go(delta: number) {
		const next = variants[(index + delta + variants.length) % variants.length];
		const url = new URL(page.url);
		url.searchParams.set('variant', next.key);
		goto(url, { replaceState: true, keepFocus: true, noScroll: true });
	}

	function setWidth(w: 'sidebar' | 'page') {
		const url = new URL(page.url);
		url.searchParams.set('width', w);
		goto(url, { replaceState: true, keepFocus: true, noScroll: true });
	}

	function onKey(e: KeyboardEvent) {
		const el = document.activeElement;
		if (el && ['INPUT', 'TEXTAREA'].includes(el.tagName)) return;
		if ((el as HTMLElement)?.isContentEditable) return;
		if (e.key === 'ArrowLeft') go(-1);
		if (e.key === 'ArrowRight') go(1);
	}
</script>

<svelte:window on:keydown={onKey} />

{#if dev}
	<div
		class="fixed bottom-6 left-1/2 z-[100] flex -translate-x-1/2 items-center gap-1 rounded-full border border-white/10 bg-zinc-900 px-2 py-1.5 text-sm text-white shadow-2xl"
	>
		<button
			type="button"
			onclick={() => go(-1)}
			class="rounded-full p-1.5 hover:bg-white/10"
			aria-label="Previous variant"><LucideChevronLeft class="size-4" /></button
		>
		<span class="min-w-[240px] px-2 text-center font-medium">
			{variants[index].key} - {variants[index].name}
		</span>
		<button
			type="button"
			onclick={() => go(1)}
			class="rounded-full p-1.5 hover:bg-white/10"
			aria-label="Next variant"><LucideChevronRight class="size-4" /></button
		>
		<span class="mx-1 h-5 w-px bg-white/15"></span>
		<button
			type="button"
			onclick={() => setWidth('sidebar')}
			class="rounded-full px-2.5 py-1 text-xs {width === 'sidebar'
				? 'bg-white text-zinc-900'
				: 'hover:bg-white/10'}">Sidebar</button
		>
		<button
			type="button"
			onclick={() => setWidth('page')}
			class="rounded-full px-2.5 py-1 text-xs {width === 'page'
				? 'bg-white text-zinc-900'
				: 'hover:bg-white/10'}">Page</button
		>
	</div>
{/if}
