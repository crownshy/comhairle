<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import { Check, Columns2 } from 'lucide-svelte';
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
	{:else if isModerator && readOnly}
		<div class="shrink-0 border-t p-4">
			<p class="text-muted-foreground text-center text-xs leading-relaxed">
				End the breakout session to continue with the agenda.
			</p>
		</div>
	{/if}
</div>

{#snippet agendaItem(item: AgendaItem, index: number)}
	{@const status = getStatus(index)}
	{@const isBreakout = item.type === 'breakout'}
	{@const wrapperClass =
		status === 'done'
			? `bg-muted-foreground/10 overflow-hidden border ${interactive ? 'hover:bg-muted-foreground/20 cursor-pointer hover:border-primary/70' : ''}`
			: status === 'current'
				? `bg-primary/40 shadow-sm ${interactive ? 'cursor-pointer' : ''}`
				: `bg-background overflow-hidden border ${interactive ? 'hover:bg-accent cursor-pointer hover:border-primary/70' : ''}`}
	<svelte:element
		this={interactive ? 'button' : 'div'}
		class="flex min-h-14 w-full {isBreakout
			? 'items-start'
			: 'items-center'} gap-2 rounded-xl px-3 py-4 text-left {wrapperClass}"
		onclick={() => handleItemClick(index)}
	>
		<!-- Number / Status icon -->
		{#if status === 'done'}
			<div
				class="bg-muted-foreground/20 flex h-6 w-6 shrink-0 items-center justify-center rounded-full"
			>
				<Check class="text-muted-foreground h-3 w-3" />
			</div>
		{:else if status === 'current'}
			<span
				class="text-primary bg-background flex h-6 shrink-0 items-center rounded-full border px-2 text-[0.7rem] font-medium"
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

		<!-- Text content -->
		<div class="flex min-w-0 flex-col gap-1">
			{#if isBreakout}
				<span
					class="flex h-6 items-center text-[0.7rem] {status === 'upcoming' &&
						'text-primary'} {status === 'done' && 'text-muted-foreground'} {status ===
						'current' && 'text-sidebar-background'}"
				>
					Breakout session
				</span>
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
