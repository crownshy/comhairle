<script lang="ts">
	// PROTOTYPE tooling: flips between UI variants of a page. Dev builds only.
	import { dev } from '$app/environment';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

	type Variant = { key: string; name: string };
	let {
		variants,
		current,
		onChange
	}: { variants: readonly Variant[]; current: string; onChange: (key: string) => void } =
		$props();

	let index = $derived(
		Math.max(
			0,
			variants.findIndex((variant) => variant.key === current)
		)
	);

	function step(delta: number) {
		const next = variants[(index + delta + variants.length) % variants.length];
		onChange(next.key);
	}

	function onKeydown(event: KeyboardEvent) {
		if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;
		if (event.metaKey || event.ctrlKey || event.altKey) return;
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, select, [contenteditable]')) return;
		event.preventDefault();
		step(event.key === 'ArrowLeft' ? -1 : 1);
	}
</script>

<svelte:window onkeydown={dev ? onKeydown : undefined} />

{#if dev}
	<div
		class="fixed bottom-4 left-1/2 z-[100] flex -translate-x-1/2 items-center gap-1 rounded-full bg-neutral-950 p-1 text-white shadow-2xl ring-2 ring-fuchsia-400"
	>
		<button
			type="button"
			class="flex size-9 items-center justify-center rounded-full hover:bg-white/15"
			onclick={() => step(-1)}
			aria-label="Previous variant"
		>
			<ChevronLeft class="size-4" />
		</button>
		<span class="px-2 text-sm whitespace-nowrap">
			<span class="font-mono text-fuchsia-300">{index + 1}/{variants.length}</span>
			{variants[index].name}
		</span>
		<button
			type="button"
			class="flex size-9 items-center justify-center rounded-full hover:bg-white/15"
			onclick={() => step(1)}
			aria-label="Next variant"
		>
			<ChevronRight class="size-4" />
		</button>
	</div>
{/if}
