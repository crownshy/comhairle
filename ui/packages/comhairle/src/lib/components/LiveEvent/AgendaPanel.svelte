<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Check } from 'lucide-svelte';
	import type { AgendaItem } from './types';

	interface Props {
		items: AgendaItem[];
		currentStep: number;
		isModerator: boolean;
		readOnly?: boolean;
		onSetCurrent?: (index: number) => void;
		onNext?: () => void;
	}

	let {
		items,
		currentStep,
		isModerator,
		readOnly = false,
		onSetCurrent,
		onNext
	}: Props = $props();

	let interactive = $derived(isModerator && !readOnly);

	function getStatus(index: number): 'done' | 'current' | 'upcoming' {
		if (index < currentStep) return 'done';
		if (index === currentStep) return 'current';
		return 'upcoming';
	}

	function handleItemClick(index: number) {
		if (!interactive) return;
		onSetCurrent?.(index);
	}

	let hasNext = $derived(currentStep < items.length - 1);
</script>

<div class="flex h-full flex-col overflow-hidden">
	<!-- Header -->
	<div class="flex shrink-0 items-center justify-center px-5">
		<h2 class="text-muted-foreground text-center text-xl leading-7 font-semibold">Agenda</h2>
	</div>

	<!-- Items (scrollable) -->
	<div class="min-h-0 flex-1 overflow-y-auto p-4">
		<div class="flex w-full flex-col gap-1.5">
			{#each items as item, index (item.id)}
				{@const status = getStatus(index)}
				{#if status === 'done'}
					<svelte:element
						this={interactive ? 'button' : 'div'}
						class="bg-muted-foreground/10 inline-flex w-full items-center overflow-hidden rounded-xl border px-3 py-4 text-left {interactive
							? 'hover:bg-muted-foreground/20 cursor-pointer'
							: ''}"
						onclick={() => handleItemClick(index)}
					>
						<div class="flex items-center gap-2">
							<div
								class="bg-muted-foreground/20 flex h-6 w-6 shrink-0 items-center justify-center rounded-full"
							>
								<Check class="text-muted-foreground h-3 w-3" />
							</div>
							<span class="text-muted-foreground line-clamp-1 text-xs font-medium">
								{item.title}
							</span>
						</div>
					</svelte:element>
				{:else if status === 'current'}
					<svelte:element
						this={interactive ? 'button' : 'div'}
						class="bg-primary/30 flex w-full flex-col items-start justify-center rounded-xl px-3 py-4 shadow-sm {interactive
							? 'cursor-pointer'
							: ''}"
						onclick={() => handleItemClick(index)}
					>
						<div class="inline-flex items-center gap-2">
							<span
								class="text-primary bg-background flex h-6 shrink-0 items-center rounded-full border px-2 text-xs font-medium"
							>
								Current
							</span>
							<span class="text-foreground line-clamp-2 text-xs font-semibold">
								{item.title}
							</span>
						</div>
					</svelte:element>
				{:else}
					<svelte:element
						this={interactive ? 'button' : 'div'}
						class="bg-background inline-flex w-full items-center overflow-hidden rounded-xl border px-3 py-4 text-left {interactive
							? 'hover:bg-accent cursor-pointer'
							: ''}"
						onclick={() => handleItemClick(index)}
					>
						<div class="flex items-center gap-2">
							<div
								class="bg-primary/10 flex h-6 w-6 shrink-0 items-center justify-center rounded-full"
							>
								<span class="text-primary text-xs leading-6 font-medium">
									{index + 1}
								</span>
							</div>
							<span class="text-foreground line-clamp-2 text-xs font-medium">
								{item.title}
							</span>
						</div>
					</svelte:element>
				{/if}
			{/each}
		</div>
	</div>

	<!-- Footer (fixed at bottom) -->
	{#if interactive}
		<div class="shrink-0 border-t p-4">
			<Button
				variant="primaryDark"
				class="h-10 w-full text-sm font-medium"
				onclick={() => onNext?.()}
				disabled={!hasNext}
			>
				Next
			</Button>
		</div>
	{/if}
</div>
