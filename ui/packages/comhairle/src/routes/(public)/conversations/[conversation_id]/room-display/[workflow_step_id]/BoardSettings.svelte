<!--
	@component The board's controls: which blocks are on, and how they are arranged.

	Lives on the display itself rather than in admin, because the question it answers
	is asked in the room. A facilitator finds the map is not landing with this
	particular group, turns it off, and the statement gets the whole wall. Making that
	a round trip through a settings page means it never happens.

	The trigger is deliberately faint and in the corner: it is not part of what the
	room is meant to read, and it should not survive into a photo of the wall. It
	comes up to full strength on hover or keyboard focus, and Cmd/Ctrl+K opens the
	panel without having to find it.
-->
<script lang="ts">
	import { Settings2, Check, Link } from '@lucide/svelte';
	import {
		ROOM_BLOCKS,
		hasBlock,
		type RoomBlock,
		type RoomBoard,
		type RoomLayout
	} from '$lib/room-display/blocks';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';
	import * as Popover from '$lib/components/ui/popover';

	type Props = {
		board: RoomBoard;
		onToggleBlock: (block: RoomBlock) => void;
		onSetLayout: (layout: RoomLayout) => void;
		onReset: () => void;
	};

	let { board, onToggleBlock, onSetLayout, onReset }: Props = $props();

	// Named for what they do to the room rather than for the component that renders
	// them, because this list is read by a facilitator, not by us.
	const LAYOUTS: { id: RoomLayout; label: string; hint: string }[] = [
		{ id: 'split', label: 'One wall', hint: 'Everything on a single surface' },
		{ id: 'console', label: 'Wall and laptop', hint: 'Controls on a second surface' },
		{ id: 'deck', label: 'Slides', hint: 'One idea at a time, you advance it' }
	];

	let open = $state(false);
	let copied = $state(false);

	function onkeydown(event: KeyboardEvent) {
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, [contenteditable]')) return;
		if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') {
			// Chrome would otherwise take this to the address bar, which on a projector
			// in fullscreen is not even visible.
			event.preventDefault();
			open = !open;
		}
	}

	async function copyLink() {
		try {
			await navigator.clipboard.writeText(window.location.href);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		} catch {
			// Clipboard access is refused outside a secure context and in some kiosk
			// setups. The URL is already in the address bar, so there is nothing to
			// recover: swallow it rather than putting an error on the wall.
		}
	}
</script>

<svelte:window {onkeydown} />

<Popover.Root bind:open>
	<Popover.Trigger
		class="text-muted-foreground hover:text-foreground focus-visible:text-foreground fixed right-4 bottom-4 z-40 rounded-full p-2 opacity-30 transition-opacity hover:opacity-100 focus-visible:opacity-100"
		aria-label="Board settings"
	>
		<Settings2 class="size-6" />
	</Popover.Trigger>

	<Popover.Content class="w-80" align="end" side="top">
		<div class="flex flex-col gap-5">
			<div class="flex items-center justify-between">
				<p class="text-foreground text-base font-semibold">Board</p>
				<Button variant="ghost" size="sm" onclick={onReset}>Reset</Button>
			</div>

			<fieldset class="flex flex-col gap-2">
				<legend class="text-muted-foreground pb-2 text-base font-medium">Arrangement</legend
				>
				{#each LAYOUTS as layout (layout.id)}
					<button
						type="button"
						class="border-border hover:bg-muted flex w-full items-start gap-3 rounded-md border px-3 py-2 text-left transition-colors"
						class:bg-muted={board.layout === layout.id}
						class:border-primary={board.layout === layout.id}
						aria-pressed={board.layout === layout.id}
						onclick={() => onSetLayout(layout.id)}
					>
						<span class="flex min-w-0 flex-1 flex-col">
							<span class="text-foreground text-base font-medium">{layout.label}</span
							>
							<span class="text-muted-foreground text-base">{layout.hint}</span>
						</span>
						{#if board.layout === layout.id}
							<Check class="text-primary mt-1 size-4 shrink-0" />
						{/if}
					</button>
				{/each}
			</fieldset>

			<fieldset class="flex flex-col gap-3">
				<legend class="text-muted-foreground pb-2 text-base font-medium">Showing</legend>
				{#each ROOM_BLOCKS as block (block.id)}
					{@const id = `board-block-${block.id}`}
					<div class="flex items-center justify-between gap-3">
						<Label for={id} class="text-foreground text-base font-normal">
							{block.label}
						</Label>
						<Switch
							{id}
							checked={hasBlock(board, block.id)}
							onCheckedChange={() => onToggleBlock(block.id)}
						/>
					</div>
				{/each}
				{#if board.layout === 'deck'}
					<!--
						Slides are advanced by hand, so there is nothing for the group buttons
						to do. Saying so beats a switch that silently does nothing.
					-->
					<p class="text-muted-foreground text-base">
						Group controls do nothing on slides: you advance them yourself.
					</p>
				{/if}
			</fieldset>

			<Button variant="outline" size="sm" onclick={copyLink}>
				{#if copied}
					<Check /> Link copied
				{:else}
					<Link /> Copy link to this board
				{/if}
			</Button>
		</div>
	</Popover.Content>
</Popover.Root>
