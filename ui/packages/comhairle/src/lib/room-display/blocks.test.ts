import { describe, expect, it } from 'vitest';
import {
	BOARD_PRESETS,
	DEFAULT_PRESET,
	hasBlock,
	isRoomBoard,
	parseBlocks,
	presetBoard,
	resolveBoard,
	ROOM_BLOCKS,
	serializeBlocks,
	toggleBlock,
	type RoomBoard
} from './blocks';

/** A board as the store would hold one, so a field added later fails loudly here. */
const stored: RoomBoard = {
	layout: 'split',
	blocks: ['map', 'qr'],
	latest: 'marquee',
	theme: 'dark'
};

describe('parseBlocks', () => {
	it('tells an absent parameter apart from an empty board', () => {
		expect(parseBlocks(null)).toBeNull();
		expect(parseBlocks(undefined)).toBeNull();
		expect(parseBlocks('')).toEqual([]);
	});

	it('drops names it does not know rather than rejecting the whole list', () => {
		expect(parseBlocks('map,sparkline,qr')).toEqual(['map', 'qr']);
	});

	it('tolerates whitespace and repeats', () => {
		expect(parseBlocks(' qr , map ,map')).toEqual(['map', 'qr']);
	});

	it('returns canonical order whatever order it was given', () => {
		expect(parseBlocks('qr,map,question')).toEqual(['question', 'map', 'qr']);
	});
});

describe('orderBlocks', () => {
	it('round-trips through serialisation', () => {
		const all = ROOM_BLOCKS.map((b) => b.id);
		expect(parseBlocks(serializeBlocks(all))).toEqual(all);
	});
});

describe('toggleBlock', () => {
	it('turns a block off and back on without disturbing the order', () => {
		const board = presetBoard('marquee');
		const without = toggleBlock(board, 'map');
		expect(hasBlock(without, 'map')).toBe(false);
		expect(toggleBlock(without, 'map').blocks).toEqual(board.blocks);
	});

	it('keeps everything that is not the block set', () => {
		const board: RoomBoard = { ...presetBoard('deck'), latest: 'column', theme: 'light' };
		const toggled = toggleBlock(board, 'qr');
		expect(toggled.layout).toBe('deck');
		expect(toggled.latest).toBe('column');
		expect(toggled.theme).toBe('light');
	});

	it('can empty the board', () => {
		let board = presetBoard('console');
		for (const id of [...board.blocks]) board = toggleBlock(board, id);
		expect(board.blocks).toEqual([]);
	});

	it('inverts the board when every block is toggled', () => {
		const board = presetBoard('console');
		let flipped = board;
		for (const { id } of ROOM_BLOCKS) flipped = toggleBlock(flipped, id);
		expect(flipped.blocks).toEqual(
			ROOM_BLOCKS.map((b) => b.id).filter((id) => !hasBlock(board, id))
		);
	});
});

describe('resolveBoard', () => {
	it('falls back to the default preset with nothing to go on', () => {
		expect(resolveBoard({})).toEqual(presetBoard(DEFAULT_PRESET));
	});

	it('ignores a preset name it does not know', () => {
		expect(resolveBoard({ preset: 'narrator' })).toEqual(presetBoard(DEFAULT_PRESET));
	});

	it('expands a preset to its layout and blocks', () => {
		expect(resolveBoard({ preset: 'marquee' })).toEqual(presetBoard('marquee'));
	});

	it('lets ?blocks= override the preset it was paired with', () => {
		const board = resolveBoard({ preset: 'marquee', blocks: 'map,qr' });
		expect(board.layout).toBe(BOARD_PRESETS.marquee.layout);
		expect(board.blocks).toEqual(['map', 'qr']);
	});

	it('lets ?layout= override on its own, keeping the preset blocks', () => {
		const board = resolveBoard({ preset: 'marquee', layout: 'console' });
		expect(board.layout).toBe('console');
		expect(board.blocks).toEqual(presetBoard('marquee').blocks);
	});

	it('honours an explicitly empty board rather than refilling it', () => {
		expect(resolveBoard({ blocks: '' }).blocks).toEqual([]);
	});

	it('comes back to what the display remembered when the URL says nothing', () => {
		expect(resolveBoard({ stored })).toEqual(stored);
	});

	it('lets any URL parameter beat what the display remembered', () => {
		// A link has to show the sender's board, not the receiving machine's leftovers.
		expect(resolveBoard({ preset: 'deck', stored })).toEqual(presetBoard('deck'));
		expect(resolveBoard({ blocks: 'question', stored }).blocks).toEqual(['question']);
		expect(resolveBoard({ layout: 'deck', stored }).layout).toBe('deck');
		expect(resolveBoard({ latest: 'column', stored }).latest).toBe('column');
		expect(resolveBoard({ theme: 'light', stored }).theme).toBe('light');
	});

	it('defaults the latest style and leaves the theme alone', () => {
		const board = resolveBoard({});
		expect(board.latest).toBe('row');
		expect(board.theme).toBe('auto');
	});

	it('reads the latest style and theme off the URL', () => {
		expect(resolveBoard({ latest: 'marquee' }).latest).toBe('marquee');
		expect(resolveBoard({ theme: 'dark' }).theme).toBe('dark');
	});

	it('ignores a latest style or theme it does not know', () => {
		expect(resolveBoard({ latest: 'ticker' }).latest).toBe('row');
		expect(resolveBoard({ theme: 'sepia' }).theme).toBe('auto');
	});

	it('does not let an unknown latest style pin the board away from storage', () => {
		// `?latest=ticker` is noise, not an instruction, so the remembered board stands.
		expect(resolveBoard({ latest: 'ticker', stored })).toEqual(stored);
	});

	it('puts a remembered board back into canonical order', () => {
		const scrambled: RoomBoard = { ...stored, blocks: ['qr', 'map', 'question'] };
		expect(resolveBoard({ stored: scrambled }).blocks).toEqual(['question', 'map', 'qr']);
	});
});

describe('isRoomBoard', () => {
	it('accepts what resolveBoard produces', () => {
		expect(isRoomBoard(resolveBoard({ preset: 'marquee' }))).toBe(true);
	});

	it('rejects anything a stale or hand-edited store might hold', () => {
		expect(isRoomBoard(null)).toBe(false);
		expect(isRoomBoard('split')).toBe(false);
		expect(isRoomBoard({ layout: 'narrator', blocks: [] })).toBe(false);
		expect(isRoomBoard({ layout: 'split' })).toBe(false);
		expect(isRoomBoard({ layout: 'split', blocks: ['map', 'sparkline'] })).toBe(false);
		// A board written before `latest` and `theme` existed.
		expect(isRoomBoard({ layout: 'split', blocks: ['map'] })).toBe(false);
		expect(isRoomBoard({ ...stored, latest: 'ticker' })).toBe(false);
		expect(isRoomBoard({ ...stored, theme: 'sepia' })).toBe(false);
	});
});
