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

/**
 * How the `marquee` block presents itself.
 *
 * `row` and `column` are still: they move only when a statement arrives. `marquee`
 * scrolls continuously, which reads badly at room distance and is kept so the two can
 * be judged against each other in an actual room rather than argued about.
 */
export type LatestStyle = 'row' | 'column' | 'marquee';

export const LATEST_STYLES = [
	{ id: 'row', label: 'Row', hint: 'Newest first, still between arrivals' },
	{ id: 'column', label: 'Column', hint: 'Top to bottom, still between arrivals' },
	{ id: 'marquee', label: 'Marquee', hint: 'Scrolls continuously' }
] as const satisfies readonly { id: LatestStyle; label: string; hint: string }[];

const LATEST_STYLE_IDS: readonly LatestStyle[] = LATEST_STYLES.map((s) => s.id);

export const DEFAULT_LATEST_STYLE: LatestStyle = 'row';

/**
 * What the room is lit like, which the display cannot work out for itself: the same
 * projector is unreadable dark in a bright hall and glaring light in a dim one.
 *
 * `auto` means "do not touch", leaving whatever the app resolved from the viewer's
 * preference. Picking light or dark drives the app-wide `themeStore`, because dark is
 * a `.dark` class on `<html>` and themed deployments key off `[data-theme=x].dark` on
 * that same element, so there is no way to scope it to this page without breaking the
 * theme. On a projector that is the right trade; on a laptop it does mean the rest of
 * the app follows, which is the same thing the site's own mode toggle does.
 */
export type RoomTheme = 'auto' | 'light' | 'dark';

export const ROOM_THEMES = [
	{ id: 'auto', label: 'Auto', hint: 'Whatever the app is set to' },
	{ id: 'light', label: 'Light', hint: 'For a bright room' },
	{ id: 'dark', label: 'Dark', hint: 'For a dim room' }
] as const satisfies readonly { id: RoomTheme; label: string; hint: string }[];

const ROOM_THEME_IDS: readonly RoomTheme[] = ROOM_THEMES.map((t) => t.id);

export interface RoomBoard {
	layout: RoomLayout;
	/** Blocks that are on, in `ROOM_BLOCKS` order so a serialised board is stable. */
	blocks: RoomBlock[];
	/** How the `marquee` block draws itself. Ignored when that block is off. */
	latest: LatestStyle;
	/** Light or dark for the room, or `auto` to leave the app's own setting alone. */
	theme: RoomTheme;
}

/**
 * The named boards, still reachable as `?variant=`. Console is the direction the team
 * picked (NOTES.md) and stays the default.
 */
export const BOARD_PRESETS = {
	console: {
		layout: 'console',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'groups', 'qr'],
		latest: 'row',
		theme: 'auto'
	},
	marquee: {
		layout: 'split',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'groups', 'qr'],
		latest: 'row',
		theme: 'auto'
	},
	deck: {
		layout: 'deck',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'qr'],
		latest: 'row',
		theme: 'auto'
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

export function isLatestStyle(value: string): value is LatestStyle {
	return (LATEST_STYLE_IDS as readonly string[]).includes(value);
}

export function isRoomTheme(value: string): value is RoomTheme {
	return (ROOM_THEME_IDS as readonly string[]).includes(value);
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
	const { layout, blocks, latest, theme } = BOARD_PRESETS[preset];
	return { layout, blocks: [...blocks], latest, theme };
}

/** Flips one block, keeping the rest in canonical order. */
export function toggleBlock(board: RoomBoard, block: RoomBlock): RoomBoard {
	const on = new Set(board.blocks);
	if (on.has(block)) on.delete(block);
	else on.add(block);
	return { ...board, blocks: orderBlocks(on) };
}

/** Anything that is not one of the three shapes we can render is not a board. */
export function isRoomBoard(value: unknown): value is RoomBoard {
	if (typeof value !== 'object' || value === null) return false;
	const candidate = value as {
		layout?: unknown;
		blocks?: unknown;
		latest?: unknown;
		theme?: unknown;
	};
	if (typeof candidate.layout !== 'string' || !isRoomLayout(candidate.layout)) return false;
	if (typeof candidate.latest !== 'string' || !isLatestStyle(candidate.latest)) return false;
	if (typeof candidate.theme !== 'string' || !isRoomTheme(candidate.theme)) return false;
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
	/** `?latest=`, how the latest-statements block draws itself. */
	latest?: string | null;
	/** `?theme=`, light or dark for the room. */
	theme?: string | null;
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
	const urlLatest = input.latest && isLatestStyle(input.latest) ? input.latest : null;
	const urlTheme = input.theme && isRoomTheme(input.theme) ? input.theme : null;
	const pinnedByUrl =
		urlBlocks !== null ||
		urlLayout !== null ||
		urlLatest !== null ||
		urlTheme !== null ||
		input.preset != null;

	const stored = !pinnedByUrl && input.stored ? input.stored : null;
	if (stored) {
		return {
			layout: stored.layout,
			blocks: orderBlocks(stored.blocks),
			latest: stored.latest,
			theme: stored.theme
		};
	}

	return {
		layout: urlLayout ?? base.layout,
		blocks: urlBlocks ?? base.blocks,
		latest: urlLatest ?? base.latest,
		theme: urlTheme ?? base.theme
	};
}
