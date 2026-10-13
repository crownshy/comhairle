/**
 * The board: which blocks are on, the layout, and the named presets, all read from and
 * written back to the URL. See CONTEXT.md, "Block / Layout / Board".
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

/**
 * The deck's slides in order, and the block each one stands for. `question`, `counts`
 * and `groups` have no slide of their own.
 */
export const DECK_SLIDES = [
	{ key: 'join', title: 'Join in', block: 'qr' },
	{ key: 'room', title: 'Who is in the room', block: 'map' },
	{ key: 'agree', title: 'What we agree on', block: 'statement' },
	{ key: 'split', title: 'Where we split', block: 'strip' },
	{ key: 'latest', title: 'Just said', block: 'marquee' }
] as const satisfies readonly { key: string; title: string; block: RoomBlock }[];

export type DeckSlideKey = (typeof DECK_SLIDES)[number]['key'];

const LAYOUTS: readonly RoomLayout[] = ['split', 'console', 'deck'];

/**
 * How the latest-statements block draws. `row`, `column` and `aside` stay still until a
 * statement arrives; `marquee` scrolls. `aside` needs the `split` layout. See NOTES.md.
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

/** Room lighting. `auto` leaves the app's own theme alone. See NOTES.md, "Lighting". */
export type RoomTheme = 'auto' | 'light' | 'dark';

export const ROOM_THEMES = [
	{ id: 'auto', label: 'Auto', hint: 'Whatever the app is set to' },
	{ id: 'light', label: 'Light', hint: 'For a bright room' },
	{ id: 'dark', label: 'Dark', hint: 'For a dim room' }
] as const satisfies readonly { id: RoomTheme; label: string; hint: string }[];

const ROOM_THEME_IDS: readonly RoomTheme[] = ROOM_THEMES.map((t) => t.id);

/**
 * How the deck's "What we agree on" and "Where we split" slides show their top five:
 * one at a time, or all together. One setting covers both slides (ADR-0044).
 */
export type SlideStyle = 'walk' | 'list';

export const SLIDE_STYLES = [
	{ id: 'walk', label: 'One at a time', hint: 'Moves through the top five; space holds one' },
	{ id: 'list', label: 'Top five', hint: 'All five on one screen' }
] as const satisfies readonly { id: SlideStyle; label: string; hint: string }[];

const SLIDE_STYLE_IDS: readonly SlideStyle[] = SLIDE_STYLES.map((s) => s.id);

export const DEFAULT_SLIDE_STYLE: SlideStyle = 'walk';

/**
 * Size multipliers. `scale` grows the whole wall and a block size grows one block on top
 * of it (`blockScale`). SizedBlock.svelte applies them. See NOTES.md, "The board".
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

/** Size names from an older panel, so links that use them still open the same board. */
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
	/** How the deck's two multi-statement slides draw themselves. Ignored elsewhere. */
	slides: SlideStyle;
}

/**
 * Named boards, reachable as `?variant=` and offered as templates in the panel. Every
 * block stays switchable after picking one.
 */
export const BOARD_PRESETS = {
	console: {
		layout: 'console',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'groups', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {},
		slides: 'walk'
	},
	wall: {
		layout: 'split',
		blocks: ['question', 'map', 'statement', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {},
		slides: 'walk'
	},
	marquee: {
		layout: 'split',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'groups', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {},
		slides: 'walk'
	},
	lobby: {
		layout: 'split',
		blocks: ['question', 'counts', 'marquee', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: { question: 1.25, qr: 2 },
		slides: 'walk'
	},
	deck: {
		layout: 'deck',
		blocks: ['question', 'counts', 'map', 'statement', 'strip', 'marquee', 'qr'],
		latest: 'row',
		theme: 'auto',
		scale: 1,
		sizes: {},
		slides: 'walk'
	},
	kiosk: {
		layout: 'split',
		blocks: ['map', 'marquee'],
		latest: 'column',
		theme: 'dark',
		scale: 1,
		sizes: { map: 1.25 },
		slides: 'walk'
	}
} as const satisfies Record<string, RoomBoard>;

export type BoardPreset = keyof typeof BOARD_PRESETS;

export const DEFAULT_PRESET: BoardPreset = 'console';

/** The presets in panel order, named for the situation rather than the layout. */
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

export function isSlideStyle(value: string): value is SlideStyle {
	return (SLIDE_STYLE_IDS as readonly string[]).includes(value);
}

function isBlockSizes(value: unknown): value is BlockSizes {
	if (typeof value !== 'object' || value === null || Array.isArray(value)) return false;
	return Object.entries(value).every(
		([block, size]) => isRoomBlock(block) && typeof size === 'number' && Number.isFinite(size)
	);
}

/** Snaps to the step and holds within range, so float noise never reaches the URL. */
export function clampScale(value: number): number {
	const snapped = Math.round(value / SCALE_STEP) * SCALE_STEP;
	const held = Math.min(MAX_SCALE, Math.max(MIN_SCALE, snapped));
	return Number(held.toFixed(2));
}

/**
 * `null` for anything that is not a number, so junk in the URL does not override the
 * remembered board. Out-of-range numbers are clamped, not dropped.
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

/** Orders keys, clamps values and drops defaults, so two equal boards serialise alike. */
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

/** Row or column for the still list. `aside` is always a column because its slot is narrow. */
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
 * `null` when the parameter is absent, which differs from an empty list: turning every
 * block off is a real choice. Unknown names are dropped so links from newer builds open.
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
	const { layout, blocks, latest, theme, scale, sizes, slides } = BOARD_PRESETS[preset];
	return {
		layout,
		blocks: [...blocks],
		latest,
		theme,
		scale,
		sizes: { ...sizes },
		slides
	};
}

/**
 * What a template fixes: layout, blocks, latest style, sizes and slides. Lighting and
 * overall scale belong to the room, so they do not affect whether a template matches.
 */
function arrangementKey(board: RoomBoard): string {
	return [
		board.layout,
		serializeBlocks(board.blocks),
		board.latest,
		serializeSizes(board.sizes),
		board.slides
	].join('|');
}

/**
 * The template the board still matches, or `null` once changed by hand. The URL only
 * carries `?variant=` while this holds.
 */
export function matchingPreset(board: RoomBoard): BoardPreset | null {
	const key = arrangementKey(board);
	for (const template of BOARD_TEMPLATES) {
		if (arrangementKey(presetBoard(template.id)) === key) return template.id;
	}
	return null;
}

/** Applies a template but keeps the room's scale, and its lighting unless the template sets one. */
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

/** Validates a board read from storage or sent by another window. */
export function isRoomBoard(value: unknown): value is RoomBoard {
	if (typeof value !== 'object' || value === null) return false;
	const candidate = value as {
		layout?: unknown;
		blocks?: unknown;
		latest?: unknown;
		theme?: unknown;
		scale?: unknown;
		sizes?: unknown;
		slides?: unknown;
	};
	if (typeof candidate.layout !== 'string' || !isRoomLayout(candidate.layout)) return false;
	if (typeof candidate.latest !== 'string' || !isLatestStyle(candidate.latest)) return false;
	if (typeof candidate.theme !== 'string' || !isRoomTheme(candidate.theme)) return false;
	if (typeof candidate.scale !== 'number' || !Number.isFinite(candidate.scale)) return false;
	if (!isBlockSizes(candidate.sizes)) return false;
	if (typeof candidate.slides !== 'string' || !isSlideStyle(candidate.slides)) return false;
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
	/** `?slides=`, `walk` or `list`, how the deck's multi-statement slides draw. */
	slides?: string | null;
	/** What this display remembered from last time, if anything. */
	stored?: RoomBoard | null;
}

/**
 * Precedence: explicit URL parameters, then the board this display remembered, then the
 * preset. A shared link must show the sender's board; a rebooted projector should not reset.
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
	const urlSlides = input.slides && isSlideStyle(input.slides) ? input.slides : null;
	const pinnedByUrl =
		urlBlocks !== null ||
		urlLayout !== null ||
		urlLatest !== null ||
		urlTheme !== null ||
		urlScale !== null ||
		urlSizes !== null ||
		urlSlides !== null ||
		input.preset != null;

	const stored = !pinnedByUrl && input.stored ? input.stored : null;
	if (stored) {
		return {
			layout: stored.layout,
			blocks: orderBlocks(stored.blocks),
			latest: stored.latest,
			theme: stored.theme,
			scale: clampScale(stored.scale),
			sizes: orderSizes(stored.sizes),
			slides: stored.slides
		};
	}

	return {
		layout: urlLayout ?? base.layout,
		blocks: urlBlocks ?? base.blocks,
		latest: urlLatest ?? base.latest,
		theme: urlTheme ?? base.theme,
		scale: urlScale ?? base.scale,
		sizes: urlSizes ?? base.sizes,
		slides: urlSlides ?? base.slides
	};
}
