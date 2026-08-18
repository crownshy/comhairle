<!-- PROTOTYPE — throwaway. Delete along with the variant branches in
     PrioritizationUser.svelte once a Continue-button placement wins (#930). -->
<script lang="ts">
	import { dev } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

	type Variant = { key: string; name: string };

	type Props = {
		variants: Variant[];
		current: string;
	};

	let { variants, current }: Props = $props();

	const index = $derived(
		Math.max(
			0,
			variants.findIndex((v) => v.key === current)
		)
	);

	function go(delta: number) {
		const next = variants[(index + delta + variants.length) % variants.length];
		const url = new URL(page.url);
		url.searchParams.set('variant', next.key);
		void goto(url, { replaceState: true, keepFocus: true, noScroll: true });
	}

	function onKeydown(event: KeyboardEvent) {
		if (!dev) return;
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, [contenteditable]')) return;
		if (event.key === 'ArrowLeft') go(-1);
		else if (event.key === 'ArrowRight') go(1);
	}
</script>

<svelte:window onkeydown={onKeydown} />

{#if dev}
	<div
		class="fixed bottom-4 left-1/2 z-50 flex -translate-x-1/2 items-center gap-1 rounded-full bg-black px-2 py-1.5 font-mono text-sm text-white shadow-lg"
	>
		<button
			type="button"
			class="rounded-full p-1 hover:bg-white/20"
			aria-label="Previous variant"
			onclick={() => go(-1)}
		>
			<ChevronLeft class="h-4 w-4" />
		</button>
		<span class="px-2 whitespace-nowrap">
			{variants[index].key} — {variants[index].name}
		</span>
		<button
			type="button"
			class="rounded-full p-1 hover:bg-white/20"
			aria-label="Next variant"
			onclick={() => go(1)}
		>
			<ChevronRight class="h-4 w-4" />
		</button>
	</div>
{/if}
