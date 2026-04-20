<script lang="ts">
	import type { AgendaItem } from './types';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { X } from 'lucide-svelte';

	interface Props {
		items: AgendaItem[];
		isFacilitator: boolean;
		onSetCurrent?: (itemId: string) => void;
		onClose: () => void;
	}

	let { items, isFacilitator, onSetCurrent, onClose }: Props = $props();
</script>

<div class="flex h-full flex-col">
	<div class="border-border flex items-center justify-between border-b px-4 py-3">
		<h2 class="text-lg font-semibold">Agenda</h2>
		<button
			class="text-muted-foreground hover:text-foreground rounded-md p-1"
			onclick={onClose}
		>
			<X class="h-4 w-4" />
		</button>
	</div>

	<div class="flex-1 overflow-y-auto p-4">
		<div class="flex flex-col gap-2">
			{#each items as item (item.id)}
				<div
					class="flex items-center justify-between rounded-lg px-4 py-3 transition-colors
						{item.isCurrent ? 'bg-primary/10 ring-primary ring-2' : 'bg-muted/50'}"
				>
					<div class="flex items-center gap-3">
						{#if item.isCurrent}
							<span class="bg-primary h-2 w-2 shrink-0 animate-pulse rounded-full"
							></span>
						{:else}
							<span class="bg-border h-2 w-2 shrink-0 rounded-full"></span>
						{/if}
						<span class="text-sm font-medium">{item.title}</span>
					</div>

					<div class="flex items-center gap-2">
						{#if item.isCurrent}
							<Badge variant="default" class="text-xs">current</Badge>
						{/if}

						{#if isFacilitator && !item.isCurrent}
							<Button
								variant="ghost"
								size="sm"
								class="h-7 text-xs"
								onclick={() => onSetCurrent?.(item.id)}
							>
								Set current
							</Button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
