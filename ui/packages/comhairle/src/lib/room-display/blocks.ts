/**
 * What the Room display is showing, and how it is arranged.
 *
 * The display used to be three hand-built variants. It is now one vocabulary of
 * blocks plus three layouts, and a variant is a named pairing of the two. That makes
 * any arrangement a link you can send someone, which is the property this route has
 * had for `?mode=` and `?question=` since the start, extended to the board itself.
 *
 * "Board" here means the arrangement. It is not the deleted `?variant=board`
 * direction, whose name this reuses now that it is free.
 *
 * Every layout reads the same block names, so a block that is off is off wherever you
 * are. What differs is how a layout spends the space:
 *
 *   block       split                console              deck
 *   question    heading              wall heading         (on the join slide)
 *   counts      line under it        console figures      (on the join slide)
 *   map         left column          wall centre          "Who is in the room"
 *   statement   right column         wall and console     "What we agree on"
 *   strip       under the statement  console picker       "Where we split"
 *   marquee     along the bottom     along the bottom     "Just said"
 *   groups      bottom bar           console list         not applicable
 *   qr          top corner           wall corner          "Join in"
 *
 * The deck is the loosest fit. It has no facilitator controls, so `groups` does
 * nothing there, and its question and counts live inside the join slide rather than
 * being separately switchable. Its other four blocks each remove a slide.
 */

export const ROOM_BLOCKS = [
	{ id: 'question', label: 'Question' },
	{ id: 'counts', label: 'Counts' },
	{ id: 'map', label: 'Opinion map' },
	{ id: 'statement', label: 'Selected statement' },
	{ id: 'strip', label: 'Statement strip' },
	{ id: 'marquee', label: 'Latest statements' },
	{ id: 'groups', label: 'Group controls' },
	{ id: 'qr', label: 'QR code' }
] as const;

export type RoomBlock = (typeof ROOM_BLOCKS)[number]['id'];

const BLOCK_ORDER: readonly RoomBlock[] = ROOM_BLOCKS.map((b) => b.id);

export type RoomLayout = 'split' | 'console' | 'deck';

const LAYOUTS: readonly RoomLayout[] = ['split', 'console', 'deck'];

export interface RoomBoard {
	layout: RoomLayout;
	/** Blocks that are on, in `ROOM_BLOCKS` order so a serialised board is stable. */
	blocks: RoomBlock[];
}

/**
 * The named boards, still reachable as `?variant=`. Console is the direction the team
 * picked (NOTES.md) and stays the default.
 */
export const BOARD_PRESETS = {
	console: {
		layout: 'console',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'groups', 'qr']
	},
	marquee: {
		layout: 'split',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'groups', 'qr']
	},
	deck: {
		layout: 'deck',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'qr']
	}
} as const satisfies Record<string, RoomBoard>;

export type BoardPreset = keyof typeof BOARD_PRESETS;

export const DEFAULT_PRESET: BoardPreset = 'console';

export function isRoomBlock(value: string): value is RoomBlock {
	return (BLOCK_ORDER as readonly string[]).includes(value);
}

export function isRoomLayout(value: string): value is RoomLayout {
	return (LAYOUTS as readonly string[]).includes(value);
}

export function isBoardPreset(value: string): value is BoardPreset {
	return Object.prototype.hasOwnProperty.call(BOARD_PRESETS, value);
}

/** Deduped and put back into canonical order, so two equal boards serialise alike. */
export function orderBlocks(blocks: Iterable<RoomBlock>): RoomBlock[] {
	const on = new Set(blocks);
	return BLOCK_ORDER.filter((b) => on.has(b));
}

export function hasBlock(board: RoomBoard, block: RoomBlock): boolean {
	return board.blocks.includes(block);
}

export function serializeBlocks(blocks: readonly RoomBlock[]): string {
	return orderBlocks(blocks).join(',');
}

/**
 * `null` means the parameter was absent, which is different from an empty board: a
 * facilitator who has turned every block off has said something, and a reload should
 * not quietly hand them the preset back. Unrecognised names are dropped rather than
 * rejected, so a link written against a later version of this list still works.
 */
export function parseBlocks(raw: string | null | undefined): RoomBlock[] | null {
	if (raw === null || raw === undefined) return null;
	return orderBlocks(
		raw
			.split(',')
			.map((part) => part.trim())
			.filter(isRoomBlock)
	);
}

export function presetBoard(preset: BoardPreset): RoomBoard {
	const { layout, blocks } = BOARD_PRESETS[preset];
	return { layout, blocks: [...blocks] };
}

/** Flips one block, keeping the rest in canonical order. */
export function toggleBlock(board: RoomBoard, block: RoomBlock): RoomBoard {
	const on = new Set(board.blocks);
	if (on.has(block)) on.delete(block);
	else on.add(block);
	return { layout: board.layout, blocks: orderBlocks(on) };
}

/** Anything that is not one of the three shapes we can render is not a board. */
export function isRoomBoard(value: unknown): value is RoomBoard {
	if (typeof value !== 'object' || value === null) return false;
	const candidate = value as { layout?: unknown; blocks?: unknown };
	if (typeof candidate.layout !== 'string' || !isRoomLayout(candidate.layout)) return false;
	if (!Array.isArray(candidate.blocks)) return false;
	return candidate.blocks.every((b) => typeof b === 'string' && isRoomBlock(b));
}

export interface ResolveBoardInput {
	/** `?variant=`, the named starting point. */
	preset?: string | null;
	/** `?layout=`, which overrides the preset's layout on its own. */
	layout?: string | null;
	/** `?blocks=`, which overrides the preset's blocks on its own. */
	blocks?: string | null;
	/** What this display remembered from last time, if anything. */
	stored?: RoomBoard | null;
}

/**
 * Precedence: an explicit URL beats what the display remembered, which beats the
 * preset. A link has to show the sender's board rather than whatever the machine at
 * the other end was last left on, or sending one is not worth doing; but a projector
 * that reboots with no parameters should come back to the setup the facilitator
 * built, not to the factory default.
 */
export function resolveBoard(input: ResolveBoardInput): RoomBoard {
	const preset = input.preset && isBoardPreset(input.preset) ? input.preset : DEFAULT_PRESET;
	const base = presetBoard(preset);

	const urlBlocks = parseBlocks(input.blocks);
	const urlLayout = input.layout && isRoomLayout(input.layout) ? input.layout : null;
	const pinnedByUrl = urlBlocks !== null || urlLayout !== null || input.preset != null;

	const stored = !pinnedByUrl && input.stored ? input.stored : null;
	if (stored) return { layout: stored.layout, blocks: orderBlocks(stored.blocks) };

	return {
		layout: urlLayout ?? base.layout,
		blocks: urlBlocks ?? base.blocks
	};
}
