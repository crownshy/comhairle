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
 *   marquee     along the bottom     wall bottom, console "Just said"
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
 * How the `marquee` block presents itself, and where it sits.
 *
 * One setting rather than a style and a placement, because only a few of the
 * combinations are worth having: a row is only readable in a wide slot, and a narrow
 * column beside the strip can only be a column.
 *
 * `row`, `column` and `aside` are still: they move only when a statement arrives.
 * `marquee` scrolls continuously, which reads badly at room distance and is kept so
 * the two can be judged against each other in an actual room rather than argued about.
 *
 * `aside` moves the block out of the band along the bottom and into the column under
 * the statement strip, which is otherwise dead space in the `split` layout. It needs
 * that column to exist, so only `split` offers it; elsewhere it falls back to a
 * column along the bottom.
 */
export type LatestStyle = 'row' | 'column' | 'marquee' | 'aside';

export const LATEST_STYLES = [
	{ id: 'row', label: 'Row', hint: 'Along the bottom, newest first', splitOnly: false },
	{ id: 'column', label: 'Column', hint: 'Along the bottom, top to bottom', splitOnly: false },
	{ id: 'aside', label: 'Beside', hint: 'Under the statement strip', splitOnly: true },
	{ id: 'marquee', label: 'Marquee', hint: 'Scrolls continuously', splitOnly: false }
] as const satisfies readonly {
	id: LatestStyle;
	label: string;
	hint: string;
	splitOnly: boolean;
}[];

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

/**
 * How big everything is, which the display cannot work out for itself either: the
 * same board is right on a meeting-room TV and too small on a hall projector, and the
 * room can only tell you once it is up.
 *
 * Two knobs. `scale` grows or shrinks the whole wall together, for the hall. A
 * per-block size grows or shrinks one region relative to the rest, for the room where
 * the question is landing but the statement is not, or where the QR code is taking
 * space the map needs. Both are plain multipliers rather than named steps: a
 * facilitator in a room wants "a bit bigger" and "a bit smaller" until it reads, and
 * a ladder of four fixed sizes kept running out at both ends. The ranges are wide on
 * purpose. Too big overflows and too small is unreadable, and both are one click
 * back, so the panel does not second-guess the person standing in the room.
 *
 * A block's effective multiplier is the two together (`blockScale`). Both are applied
 * as CSS custom properties rather than by picking classes, so every text and spacing
 * utility inside a block moves and none of the markup knows about it.
 */
export const MIN_SCALE = 0.5;
export const MAX_SCALE = 3;
export const SCALE_STEP = 0.1;
export const DEFAULT_SCALE = 1;

export const MIN_BLOCK_SIZE = 0.25;
export const MAX_BLOCK_SIZE = 4;
export const BLOCK_SIZE_STEP = 0.25;
export const DEFAULT_BLOCK_SIZE = 1;

/** Only the blocks that are not at the default, so a board with nothing set is `{}`. */
export type BlockSizes = Partial<Record<RoomBlock, number>>;

/**
 * The size names the panel used to offer, so a link written against them still opens
 * the same board.
 */
const LEGACY_BLOCK_SIZES: Record<string, number> = { m: 1, l: 1.25, xl: 1.5, xxl: 2 };

export interface RoomBoard {
	layout: RoomLayout;
	/** Blocks that are on, in `ROOM_BLOCKS` order so a serialised board is stable. */
	blocks: RoomBlock[];
	/** How the `marquee` block draws itself. Ignored when that block is off. */
	latest: LatestStyle;
	/** Light or dark for the room, or `auto` to leave the app's own setting alone. */
	theme: RoomTheme;
	/** Multiplier on everything, `MIN_SCALE` to `MAX_SCALE`. */
	scale: number;
	/** Per-block multipliers on top of `scale`; a block that is absent is normal. */
	sizes: BlockSizes;
}

/**
 * The named boards, reachable as `?variant=` and offered as templates in the panel.
 * Each one is a whole board for a situation a facilitator recognises, and is only a
 * starting point: every block stays switchable after picking one. Console is the
 * direction the team picked (NOTES.md) and stays the default.
 *
 * The older names (`console`, `marquee`, `deck`) are kept so links written against
 * them still open the same board.
 */
export const BOARD_PRESETS = {
	console: {
		layout: 'console',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'groups', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {}
	},
	wall: {
		layout: 'split',
		blocks: ['question', 'map', 'statement', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {}
	},
	marquee: {
		layout: 'split',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'groups', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {}
	},
	lobby: {
		layout: 'split',
		blocks: ['question', 'counts', 'marquee', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: { question: 1.25, qr: 2 }
	},
	deck: {
		layout: 'deck',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {}
	},
	kiosk: {
		layout: 'split',
		blocks: ['map', 'marquee'],
		latest: 'column',
		theme: 'dark',
		scale: 1,
		sizes: { map: 1.25 }
	}
} as const satisfies Record<string, RoomBoard>;

export type BoardPreset = keyof typeof BOARD_PRESETS;

export const DEFAULT_PRESET: BoardPreset = 'console';

/**
 * The presets as the panel lists them, in the order a facilitator is likely to want
 * them: the common case first, the unattended screen last. Named for the situation
 * rather than the layout, because the person choosing knows what room they are in and
 * not what a "split" is.
 */
export const BOARD_TEMPLATES = [
	{
		id: 'console',
		label: 'Facilitated room',
		hint: 'Wall and laptop; you point the wall from the console'
	},
	{
		id: 'wall',
		label: 'Wall only',
		hint: 'One screen and no laptop: question, map, statement, QR code'
	},
	{
		id: 'marquee',
		label: 'One wall, everything',
		hint: 'Map, statements and group controls on a single screen'
	},
	{
		id: 'lobby',
		label: 'Lobby',
		hint: 'People arriving: question, counts, latest statements, big QR code'
	},
	{ id: 'deck', label: 'Slides', hint: 'One idea at a time, you advance it' },
	{
		id: 'kiosk',
		label: 'Kiosk',
		hint: 'Left running unattended: map and latest statements, dark'
	}
] as const satisfies readonly { id: BoardPreset; label: string; hint: string }[];

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

function isBlockSizes(value: unknown): value is BlockSizes {
	if (typeof value !== 'object' || value === null || Array.isArray(value)) return false;
	return Object.entries(value).every(
		([block, size]) => isRoomBlock(block) && typeof size === 'number' && Number.isFinite(size)
	);
}

/**
 * Snapped to the slider's step and held inside its range, so `1.4000000000000001`
 * never reaches the URL and `?scale=40` is a big wall rather than a broken one.
 */
export function clampScale(value: number): number {
	const snapped = Math.round(value / SCALE_STEP) * SCALE_STEP;
	const held = Math.min(MAX_SCALE, Math.max(MIN_SCALE, snapped));
	return Number(held.toFixed(2));
}

/**
 * `null` for anything that is not a number, so a stray `?scale=big` is noise rather
 * than an instruction and does not pin the board away from what the display
 * remembered. A number out of range is still a number: it is clamped, not dropped.
 */
export function parseScale(raw: string | null | undefined): number | null {
	if (raw === null || raw === undefined || raw.trim() === '') return null;
	const value = Number(raw);
	return Number.isFinite(value) ? clampScale(value) : null;
}

/** Same treatment as `clampScale`: snapped to the stepper's step and held in range. */
export function clampBlockSize(value: number): number {
	const snapped = Math.round(value / BLOCK_SIZE_STEP) * BLOCK_SIZE_STEP;
	const held = Math.min(MAX_BLOCK_SIZE, Math.max(MIN_BLOCK_SIZE, snapped));
	return Number(held.toFixed(2));
}

/** A multiplier, or one of the old size names; `null` for anything else. */
export function parseBlockSize(raw: string): number | null {
	const legacy = LEGACY_BLOCK_SIZES[raw];
	if (legacy !== undefined) return legacy;
	if (raw.trim() === '') return null;
	const value = Number(raw);
	return Number.isFinite(value) ? clampBlockSize(value) : null;
}

export function blockSize(board: RoomBoard, block: RoomBlock): number {
	return board.sizes[block] ?? DEFAULT_BLOCK_SIZE;
}

/** Sets one block's multiplier, dropping the entry when it is back at normal. */
export function setBlockSize(board: RoomBoard, block: RoomBlock, size: number): RoomBoard {
	return { ...board, sizes: orderSizes({ ...board.sizes, [block]: size }) };
}

/** What one block is actually multiplied by: the wall's scale times its own size. */
export function blockScale(board: RoomBoard, block: RoomBlock): number {
	return Number((board.scale * blockSize(board, block)).toFixed(3));
}

/**
 * Keys in `ROOM_BLOCKS` order, every value held in range and defaults dropped, so two
 * equal boards serialise alike and a remembered `map: 40` is a big map, not a broken one.
 */
function orderSizes(sizes: BlockSizes): BlockSizes {
	const ordered: BlockSizes = {};
	for (const block of BLOCK_ORDER) {
		const size = sizes[block];
		if (size === undefined) continue;
		const held = clampBlockSize(size);
		if (held !== DEFAULT_BLOCK_SIZE) ordered[block] = held;
	}
	return ordered;
}

/** `question:1.25,statement:2`; empty when every block is normal. */
export function serializeSizes(sizes: BlockSizes): string {
	return Object.entries(orderSizes(sizes))
		.map(([block, size]) => `${block}:${size}`)
		.join(',');
}

/**
 * Same contract as `parseBlocks`: `null` for an absent parameter, an empty map for an
 * empty one, and pairs it does not recognise dropped rather than rejected.
 */
export function parseSizes(raw: string | null | undefined): BlockSizes | null {
	if (raw === null || raw === undefined) return null;
	const sizes: BlockSizes = {};
	for (const part of raw.split(',')) {
		const [block, rawSize] = part.split(':').map((s) => s.trim());
		if (!block || !rawSize || !isRoomBlock(block)) continue;
		const size = parseBlockSize(rawSize);
		if (size !== null) sizes[block] = size;
	}
	return orderSizes(sizes);
}

/**
 * Which way the still list runs, for the layouts that have already decided they are
 * not drawing the marquee. `aside` says where the block goes rather than how it looks,
 * and what goes there is a column: a row of four in a third of the wall's width would
 * be four slivers.
 */
export function stillLatestDirection(board: RoomBoard): 'row' | 'column' {
	return board.latest === 'row' ? 'row' : 'column';
}

/** Whether the block sits in the column under the strip rather than along the bottom. */
export function latestIsBeside(board: RoomBoard): boolean {
	return board.latest === 'aside' && board.layout === 'split';
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
	const { layout, blocks, latest, theme, scale, sizes } = BOARD_PRESETS[preset];
	return { layout, blocks: [...blocks], latest, theme, scale, sizes: { ...sizes } };
}

/**
 * A template is the arrangement: layout, blocks, how the latest statements draw and
 * the per-block sizes. Lighting and the overall scale belong to the room instead: the
 * projector is set up once for the hall, and choosing a different template for the
 * afternoon session should not undo that, nor should having set it make the panel
 * say the board is no longer the template it plainly is.
 */
function arrangementKey(board: RoomBoard): string {
	return [
		board.layout,
		serializeBlocks(board.blocks),
		board.latest,
		serializeSizes(board.sizes)
	].join('|');
}

/**
 * Which template the board still is, or `null` once it has been changed by hand. The
 * panel shows this in its template picker, and the URL carries it as `?variant=` only
 * while it is true: a link that names a template has to open that template.
 */
export function matchingPreset(board: RoomBoard): BoardPreset | null {
	const key = arrangementKey(board);
	for (const template of BOARD_TEMPLATES) {
		if (arrangementKey(presetBoard(template.id)) === key) return template.id;
	}
	return null;
}

/**
 * The template's arrangement on top of this room's lighting and scale. A template
 * that names a lighting (kiosk is dark) still gets it; `auto` means it has no opinion.
 */
export function applyPreset(board: RoomBoard, preset: BoardPreset): RoomBoard {
	const next = presetBoard(preset);
	return {
		...next,
		theme: next.theme === 'auto' ? board.theme : next.theme,
		scale: board.scale
	};
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
		scale?: unknown;
		sizes?: unknown;
	};
	if (typeof candidate.layout !== 'string' || !isRoomLayout(candidate.layout)) return false;
	if (typeof candidate.latest !== 'string' || !isLatestStyle(candidate.latest)) return false;
	if (typeof candidate.theme !== 'string' || !isRoomTheme(candidate.theme)) return false;
	if (typeof candidate.scale !== 'number' || !Number.isFinite(candidate.scale)) return false;
	if (!isBlockSizes(candidate.sizes)) return false;
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
	/** `?scale=`, the multiplier on the whole wall. */
	scale?: string | null;
	/** `?sizes=`, per-block steps as `block:size` pairs. */
	sizes?: string | null;
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
	const urlScale = parseScale(input.scale);
	const urlSizes = parseSizes(input.sizes);
	const pinnedByUrl =
		urlBlocks !== null ||
		urlLayout !== null ||
		urlLatest !== null ||
		urlTheme !== null ||
		urlScale !== null ||
		urlSizes !== null ||
		input.preset != null;

	const stored = !pinnedByUrl && input.stored ? input.stored : null;
	if (stored) {
		return {
			layout: stored.layout,
			blocks: orderBlocks(stored.blocks),
			latest: stored.latest,
			theme: stored.theme,
			scale: clampScale(stored.scale),
			sizes: orderSizes(stored.sizes)
		};
	}

	return {
		layout: urlLayout ?? base.layout,
		blocks: urlBlocks ?? base.blocks,
		latest: urlLatest ?? base.latest,
		theme: urlTheme ?? base.theme,
		scale: urlScale ?? base.scale,
		sizes: urlSizes ?? base.sizes
	};
}
