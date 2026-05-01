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
		onEndMeeting?: () => void;
	}

	let {
		items,
		currentStep,
		isModerator,
		readOnly = false,
		onSetCurrent,
		onNext,
		onEndMeeting
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
	let isLastItem = $derived(currentStep === items.length - 1 && items.length > 0);
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
				{@render agendaItem(item, index)}
			{/each}
		</div>
	</div>

	<!-- Footer (fixed at bottom) -->
	{#if interactive}
		<div class="shrink-0 border-t p-4">
			{#if isLastItem}
				<Button
					variant="destructive"
					class="h-10 w-full text-sm font-medium"
					onclick={() => onEndMeeting?.()}
				>
					End meeting
				</Button>
			{:else}
				<Button
					variant="primaryDark"
					class="h-10 w-full text-sm font-medium"
					onclick={() => onNext?.()}
					disabled={!hasNext}
				>
					Next
				</Button>
			{/if}
		</div>
	{/if}
</div>

{#snippet agendaItem(item: AgendaItem, index: number)}
	{@const status = getStatus(index)}
	{@const wrapperClass =
		status === 'done'
			? `bg-muted-foreground/10 inline-flex items-center overflow-hidden border ${interactive ? 'hover:bg-muted-foreground/20 cursor-pointer' : ''}`
			: status === 'current'
				? `bg-primary/30 flex flex-col items-start justify-center shadow-sm ${interactive ? 'cursor-pointer' : ''}`
				: `bg-background inline-flex items-center overflow-hidden border ${interactive ? 'hover:bg-accent cursor-pointer' : ''}`}
	<svelte:element
		this={interactive ? 'button' : 'div'}
		class="w-full rounded-xl px-3 py-4 text-left {wrapperClass}"
		onclick={() => handleItemClick(index)}
	>
		<div class="{status === 'current' ? 'inline-flex' : 'flex'} items-center gap-2">
			{#if status === 'done'}
				<div
					class="bg-muted-foreground/20 flex h-6 w-6 shrink-0 items-center justify-center rounded-full"
				>
					<Check class="text-muted-foreground h-3 w-3" />
				</div>
			{:else if status === 'current'}
				<span
					class="text-primary bg-background flex h-6 shrink-0 items-center rounded-full border px-2 text-xs font-medium"
				>
					Current
				</span>
			{:else}
				<div
					class="bg-primary/10 flex h-6 w-6 shrink-0 items-center justify-center rounded-full"
				>
					<span class="text-primary text-xs leading-6 font-medium">
						{index + 1}
					</span>
				</div>
			{/if}
			<span
				class="text-xs {status === 'done'
					? 'text-muted-foreground line-clamp-1 font-medium'
					: status === 'current'
						? 'text-foreground line-clamp-2 font-semibold'
						: 'text-foreground line-clamp-2 font-medium'}"
			>
				{item.title}
			</span>
		</div>
	</svelte:element>
{/snippet}
