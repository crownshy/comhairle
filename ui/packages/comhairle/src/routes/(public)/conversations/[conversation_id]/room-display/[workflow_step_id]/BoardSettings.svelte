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
		BLOCK_SIZE_STEP,
		BOARD_TEMPLATES,
		DECK_SLIDES,
		LATEST_STYLES,
		MAX_BLOCK_SIZE,
		MAX_SCALE,
		MIN_BLOCK_SIZE,
		MIN_SCALE,
		ROOM_BLOCKS,
		ROOM_THEMES,
		SCALE_STEP,
		SLIDE_STYLES,
		blockSize,
		clampScale,
		hasBlock,
		isBoardPreset,
		matchingPreset,
		type BoardPreset,
		type LatestStyle,
		type RoomBlock,
		type RoomBoard,
		type RoomLayout,
		type RoomTheme,
		type SlideStyle
	} from '$lib/room-display/blocks';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { Label } from '$lib/components/ui/label';
	import * as Popover from '$lib/components/ui/popover';
	import * as Select from '$lib/components/ui/select';
	import SizeStepper from './SizeStepper.svelte';

	type Props = {
		board: RoomBoard;
		onSetPreset: (preset: BoardPreset) => void;
		onToggleBlock: (block: RoomBlock) => void;
		onSetLayout: (layout: RoomLayout) => void;
		onSetLatest: (latest: LatestStyle) => void;
		onSetTheme: (theme: RoomTheme) => void;
		onSetScale: (scale: number) => void;
		onSetBlockSize: (block: RoomBlock, size: number) => void;
		onSetSlideStyle: (style: SlideStyle) => void;
		onReset: () => void;
	};

	let {
		board,
		onSetPreset,
		onToggleBlock,
		onSetLayout,
		onSetLatest,
		onSetTheme,
		onSetScale,
		onSetBlockSize,
		onSetSlideStyle,
		onReset
	}: Props = $props();

	// Named for what they do to the room rather than for the component that renders
	// them, because this list is read by a facilitator, not by us.
	const LAYOUTS: { id: RoomLayout; label: string; hint: string }[] = [
		{ id: 'split', label: 'One wall', hint: 'Everything on a single surface' },
		{ id: 'console', label: 'Wall and laptop', hint: 'Controls on a second surface' },
		{ id: 'deck', label: 'Slides', hint: 'One idea at a time, you advance it' }
	];

	let open = $state(false);
	let copied = $state(false);

	// The template the board still is. Anything touched by hand is "Custom", which is
	// not an option in the list: there is nothing to pick it back to.
	const template = $derived(matchingPreset(board));
	const templateLabel = $derived(
		BOARD_TEMPLATES.find((option) => option.id === template)?.label ?? 'Custom'
	);

	// On the deck the panel lists slides, by the name on the wall, rather than blocks:
	// a facilitator running slides has no "statement strip" to size, they have "Where
	// we split". Blocks with no slide (question, counts, group controls) are not shown,
	// because a switch that does nothing is worse than no switch.
	const onDeck = $derived(board.layout === 'deck');
	const rows = $derived<{ id: RoomBlock; label: string }[]>(
		onDeck
			? DECK_SLIDES.map((slide) => ({ id: slide.block, label: slide.title }))
			: ROOM_BLOCKS.map((block) => ({ id: block.id, label: block.label }))
	);

	// "Beside" means the column under the statement strip, which only the one-wall
	// layout has. Offering it elsewhere would be a button that silently does something
	// else.
	const latestOptions = $derived(
		LATEST_STYLES.filter((option) => !option.splitOnly || board.layout === 'split')
	);

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

	<Popover.Content class="max-h-[85vh] w-96 overflow-y-auto" align="end" side="top">
		<div class="flex flex-col gap-5">
			<div class="flex items-center justify-between">
				<p class="text-foreground text-base font-semibold">Board</p>
				<Button variant="ghost" size="sm" onclick={onReset}>Reset</Button>
			</div>

			<!--
				A whole board in one pick, for the facilitator who knows what room they are
				in and does not want to assemble it block by block. Everything below still
				applies afterwards; the pick is where you start, not a mode.
			-->
			<div class="flex flex-col gap-2">
				<Label for="board-template" class="text-muted-foreground text-base font-medium">
					Template
				</Label>
				<Select.Root
					type="single"
					value={template ?? ''}
					onValueChange={(value) => {
						if (value && isBoardPreset(value)) onSetPreset(value);
					}}
				>
					<Select.Trigger id="board-template" class="w-full text-base">
						{templateLabel}
					</Select.Trigger>
					<!-- As wide as the trigger and no wider, so the hints wrap instead of the list growing past the panel. -->
					<Select.Content class="w-(--bits-select-anchor-width)">
						{#each BOARD_TEMPLATES as option (option.id)}
							<!-- The item centres its last span by default, which is meant for an icon, not two lines of text. -->
							<Select.Item value={option.id} label={option.label} class="py-2">
								<span class="flex min-w-0 flex-col items-start! text-left">
									<span class="text-foreground text-base">{option.label}</span>
									<span class="text-muted-foreground text-base"
										>{option.hint}</span
									>
								</span>
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
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

			<fieldset class="flex flex-col gap-2">
				<legend class="text-muted-foreground pb-2 text-base font-medium">Lighting</legend>
				<div class="flex gap-2">
					{#each ROOM_THEMES as option (option.id)}
						<button
							type="button"
							class="border-border hover:bg-muted flex-1 rounded-md border px-2 py-2 text-base transition-colors"
							class:bg-muted={board.theme === option.id}
							class:border-primary={board.theme === option.id}
							aria-pressed={board.theme === option.id}
							title={option.hint}
							onclick={() => onSetTheme(option.id)}
						>
							{option.label}
						</button>
					{/each}
				</div>
			</fieldset>

			{#if onDeck}
				<!--
					One choice for both statement slides, not one each: "walk them through it or
					show them the result" is a decision about the session. Only the deck has
					these slides, so only the deck shows the choice.
				-->
				<fieldset class="flex flex-col gap-2">
					<legend class="text-muted-foreground pb-2 text-base font-medium">
						Statement slides show
					</legend>
					<div class="flex gap-2">
						{#each SLIDE_STYLES as option (option.id)}
							<button
								type="button"
								class="border-border hover:bg-muted flex-1 rounded-md border px-2 py-2 text-base transition-colors"
								class:bg-muted={board.slides === option.id}
								class:border-primary={board.slides === option.id}
								aria-pressed={board.slides === option.id}
								title={option.hint}
								onclick={() => onSetSlideStyle(option.id)}
							>
								{option.label}
							</button>
						{/each}
					</div>
				</fieldset>
			{/if}

			<!--
				One row per block: whether it is on, and how big. The two used to be separate
				lists, which meant reading every block name twice to set up one wall.
				"Everything" comes first because it is what you reach for when the back row
				cannot read any of it.
			-->
			<fieldset class="grid grid-cols-[1fr_auto_auto] items-center gap-x-3 gap-y-3">
				<legend class="text-muted-foreground pb-2 text-base font-medium">
					{onDeck ? 'Slides' : 'Blocks'}
				</legend>
				<span class="text-foreground text-base">Everything</span>
				<SizeStepper
					value={board.scale}
					min={MIN_SCALE}
					max={MAX_SCALE}
					step={SCALE_STEP}
					label="everything"
					onchange={(value) => onSetScale(clampScale(value))}
				/>
				<!-- Nothing to switch off: the empty cell keeps the steppers in one column. -->
				<span></span>
				{#each rows as block (block.id)}
					{@const id = `board-block-${block.id}`}
					{@const on = hasBlock(board, block.id)}
					<Label for={id} class="text-foreground min-w-0 text-base font-normal">
						{block.label}
					</Label>
					<!-- A block that is off has nothing to size. The stepper keeps its space so the switches line up. -->
					<div class:invisible={!on}>
						<SizeStepper
							value={blockSize(board, block.id)}
							min={MIN_BLOCK_SIZE}
							max={MAX_BLOCK_SIZE}
							step={BLOCK_SIZE_STEP}
							label={block.label}
							onchange={(value) => onSetBlockSize(block.id, value)}
						/>
					</div>
					<Switch {id} checked={on} onCheckedChange={() => onToggleBlock(block.id)} />
					{#if block.id === 'marquee' && on && !onDeck}
						<!--
							Nested under the block it belongs to: it is how that block draws
							itself, not a ninth thing to switch on.
						-->
						<div
							class="border-border col-span-3 flex flex-col gap-2 border-l pb-1 pl-4"
						>
							<p class="text-muted-foreground text-base">Latest statements as</p>
							<div class="flex flex-wrap gap-2">
								{#each latestOptions as option (option.id)}
									<button
										type="button"
										class="border-border hover:bg-muted min-w-16 flex-1 rounded-md border px-2 py-2 text-base transition-colors"
										class:bg-muted={board.latest === option.id}
										class:border-primary={board.latest === option.id}
										aria-pressed={board.latest === option.id}
										title={option.hint}
										onclick={() => onSetLatest(option.id)}
									>
										{option.label}
									</button>
								{/each}
							</div>
							{#if board.latest === 'marquee'}
								<p class="text-muted-foreground text-base">
									Scrolling text is hard to read across a room. Kept so you can
									judge it against the still versions in the room itself.
								</p>
							{/if}
						</div>
					{/if}
				{/each}
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
