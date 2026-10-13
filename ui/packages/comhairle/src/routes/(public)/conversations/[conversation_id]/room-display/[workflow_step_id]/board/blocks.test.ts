import { describe, expect, it } from 'vitest';
import {
	BOARD_PRESETS,
	BOARD_TEMPLATES,
	DEFAULT_PRESET,
	MAX_BLOCK_SIZE,
	MAX_SCALE,
	MIN_BLOCK_SIZE,
	MIN_SCALE,
	blockScale,
	blockSize,
	clampBlockSize,
	clampScale,
	hasBlock,
	applyPreset,
	isRoomBoard,
	matchingPreset,
	parseBlocks,
	parseScale,
	parseSizes,
	presetBoard,
	resolveBoard,
	ROOM_BLOCKS,
	stillLatestDirection,
	latestIsBeside,
	serializeBlocks,
	serializeSizes,
	setBlockSize,
	toggleBlock,
	type RoomBoard
} from './blocks';

/** A board as the store would hold one, so a field added later fails loudly here. */
const stored: RoomBoard = {
	layout: 'split',
	blocks: ['map', 'qr'],
	latest: 'marquee',
	theme: 'dark',
	scale: 1.4,
	sizes: { map: 1.25 },
	slides: 'list'
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

	it('defaults to normal size with nothing set', () => {
		const board = resolveBoard({});
		expect(board.scale).toBe(1);
		expect(board.sizes).toEqual({});
	});

	it('reads the scale and sizes off the URL', () => {
		const board = resolveBoard({ scale: '1.5', sizes: 'question:1.25,statement:2' });
		expect(board.scale).toBe(1.5);
		expect(board.sizes).toEqual({ question: 1.25, statement: 2 });
	});

	it('still opens a link written against the old size names', () => {
		const board = resolveBoard({ sizes: 'question:l,statement:xxl,qr:m' });
		expect(board.sizes).toEqual({ question: 1.25, statement: 2 });
	});

	it('lets a scale or a size pin the board away from storage', () => {
		expect(resolveBoard({ scale: '1.2', stored }).sizes).toEqual({});
		expect(resolveBoard({ sizes: 'qr:1.5', stored }).scale).toBe(1);
	});

	it('treats a scale that is not a number as noise', () => {
		expect(resolveBoard({ scale: 'big', stored })).toEqual(stored);
		expect(resolveBoard({ scale: 'big' }).scale).toBe(1);
	});

	it('holds a remembered scale and size inside the range', () => {
		expect(resolveBoard({ stored: { ...stored, scale: 9 } }).scale).toBe(MAX_SCALE);
		expect(resolveBoard({ stored: { ...stored, scale: 0.2 } }).scale).toBe(MIN_SCALE);
		expect(resolveBoard({ stored: { ...stored, sizes: { map: 40 } } }).sizes).toEqual({
			map: MAX_BLOCK_SIZE
		});
	});
});

describe('scale', () => {
	it('snaps to the slider step and stays in range', () => {
		expect(clampScale(1.4000000000000001)).toBe(1.4);
		expect(clampScale(1.33)).toBe(1.3);
		expect(clampScale(40)).toBe(MAX_SCALE);
		expect(clampScale(0)).toBe(MIN_SCALE);
	});

	it('parses a number and rejects anything else', () => {
		expect(parseScale(null)).toBeNull();
		expect(parseScale('')).toBeNull();
		expect(parseScale('big')).toBeNull();
		expect(parseScale('1.2')).toBe(1.2);
		// Out of range is still an instruction, just a clamped one.
		expect(parseScale('7')).toBe(MAX_SCALE);
	});

	it('multiplies the wall scale by the block step', () => {
		const board: RoomBoard = { ...presetBoard('console'), scale: 1.2, sizes: { map: 2 } };
		expect(blockScale(board, 'map')).toBe(2.4);
		expect(blockScale(board, 'question')).toBe(1.2);
	});

	it('lets a block shrink below normal', () => {
		const board: RoomBoard = { ...presetBoard('console'), scale: 2, sizes: { qr: 0.5 } };
		expect(blockScale(board, 'qr')).toBe(1);
	});
});

describe('block sizes', () => {
	it('tells an absent parameter apart from an empty map', () => {
		expect(parseSizes(null)).toBeNull();
		expect(parseSizes('')).toEqual({});
	});

	it('drops pairs it does not know rather than rejecting the list', () => {
		expect(parseSizes('map:1.25,sparkline:2,qr:huge,question,strip:')).toEqual({
			map: 1.25
		});
	});

	it('serialises in canonical order without defaults', () => {
		expect(serializeSizes({ qr: 1.5, question: 1.25, map: 1 })).toBe('question:1.25,qr:1.5');
		expect(serializeSizes({})).toBe('');
	});

	it('round-trips through the URL', () => {
		const sizes = { question: 1.25, statement: 2, qr: 0.5 };
		expect(parseSizes(serializeSizes(sizes))).toEqual(sizes);
	});

	it('snaps to the stepper and stays in range', () => {
		expect(clampBlockSize(1.3)).toBe(1.25);
		expect(clampBlockSize(0.1)).toBe(MIN_BLOCK_SIZE);
		expect(clampBlockSize(40)).toBe(MAX_BLOCK_SIZE);
		expect(parseSizes('map:0')).toEqual({ map: MIN_BLOCK_SIZE });
	});

	it('is normal for any block that is not set', () => {
		expect(blockSize(presetBoard('console'), 'map')).toBe(1);
		expect(blockSize(stored, 'map')).toBe(1.25);
	});

	it('drops the entry when a block goes back to normal', () => {
		const grown = setBlockSize(presetBoard('console'), 'strip', 1.5);
		expect(grown.sizes).toEqual({ strip: 1.5 });
		expect(setBlockSize(grown, 'strip', 1).sizes).toEqual({});
	});

	it('goes below normal as well as above', () => {
		expect(setBlockSize(presetBoard('console'), 'qr', 0.5).sizes).toEqual({ qr: 0.5 });
	});

	it('leaves the rest of the board alone', () => {
		const sized = setBlockSize(stored, 'qr', 2);
		expect(sized.blocks).toEqual(stored.blocks);
		expect(sized.scale).toBe(stored.scale);
		expect(sized.sizes).toEqual({ map: 1.25, qr: 2 });
	});
});

describe('latest placement', () => {
	it('draws aside as a column, since that is what fits under the strip', () => {
		const board: RoomBoard = { ...presetBoard('marquee'), latest: 'aside' };
		expect(stillLatestDirection(board)).toBe('column');
	});

	it('is a row only when the style is a row', () => {
		expect(stillLatestDirection({ ...presetBoard('marquee'), latest: 'row' })).toBe('row');
		expect(stillLatestDirection({ ...presetBoard('marquee'), latest: 'column' })).toBe(
			'column'
		);
	});

	it('only sits beside the strip on the one layout that has one', () => {
		expect(latestIsBeside({ ...presetBoard('marquee'), latest: 'aside' })).toBe(true);
		expect(latestIsBeside({ ...presetBoard('console'), latest: 'aside' })).toBe(false);
		expect(latestIsBeside({ ...presetBoard('deck'), latest: 'aside' })).toBe(false);
	});

	it('is not beside anything when the style is not aside', () => {
		expect(latestIsBeside(presetBoard('marquee'))).toBe(false);
	});

	it('survives a round trip through the URL', () => {
		expect(resolveBoard({ preset: 'marquee', latest: 'aside' }).latest).toBe('aside');
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
		// A board written before `scale` and `sizes` existed.
		expect(
			isRoomBoard({ layout: 'split', blocks: ['map'], latest: 'row', theme: 'auto' })
		).toBe(false);
		expect(isRoomBoard({ ...stored, scale: 'big' })).toBe(false);
		expect(isRoomBoard({ ...stored, sizes: { map: 'huge' } })).toBe(false);
		expect(isRoomBoard({ ...stored, sizes: { sparkline: 1.25 } })).toBe(false);
		// A board remembered while sizes were still names. Rebuilt rather than translated.
		expect(isRoomBoard({ ...stored, sizes: { map: 'l' } })).toBe(false);
		expect(isRoomBoard({ ...stored, sizes: ['map'] })).toBe(false);
		// A board written before `slides` existed.
		expect(isRoomBoard({ ...stored, slides: undefined })).toBe(false);
		expect(isRoomBoard({ ...stored, slides: 'carousel' })).toBe(false);
		// A board remembered while the style was still per slide.
		expect(isRoomBoard({ ...stored, slides: { statement: 'list' } })).toBe(false);
	});
});

describe('slide style', () => {
	it('walks by default, on every template', () => {
		for (const template of BOARD_TEMPLATES) {
			expect(presetBoard(template.id).slides).toBe('walk');
		}
	});

	it('reads the style off the URL, and lets it pin the board', () => {
		expect(resolveBoard({ preset: 'deck', slides: 'list' }).slides).toBe('list');
		expect(resolveBoard({ slides: 'walk', stored }).slides).toBe('walk');
		expect(resolveBoard({ stored }).slides).toBe('list');
	});

	it('treats a style it does not know as noise', () => {
		expect(resolveBoard({ slides: 'carousel' }).slides).toBe('walk');
		expect(resolveBoard({ slides: 'carousel', stored })).toEqual(stored);
		// The old per-slide form is not translated: the pair is not a style.
		expect(resolveBoard({ slides: 'statement:list', stored })).toEqual(stored);
	});

	it('is part of the arrangement, so changing it leaves the template', () => {
		expect(matchingPreset(presetBoard('deck'))).toBe('deck');
		expect(matchingPreset({ ...presetBoard('deck'), slides: 'list' })).toBeNull();
	});
});

describe('templates', () => {
	it('lists every preset exactly once, so nothing is reachable by link but not by panel', () => {
		const listed = BOARD_TEMPLATES.map((t) => t.id).sort();
		expect(listed).toEqual(Object.keys(BOARD_PRESETS).sort());
	});

	it('recognises each template from the board it expands to', () => {
		for (const template of BOARD_TEMPLATES) {
			expect(matchingPreset(presetBoard(template.id))).toBe(template.id);
		}
	});

	it('stops naming a template once the arrangement is touched by hand', () => {
		expect(matchingPreset(toggleBlock(presetBoard('console'), 'qr'))).toBeNull();
		expect(matchingPreset({ ...presetBoard('console'), latest: 'marquee' })).toBeNull();
		expect(matchingPreset(setBlockSize(presetBoard('wall'), 'map', 1.5))).toBeNull();
	});

	it('still names the template when only the room has changed', () => {
		// Lighting and scale are set for the hall, not chosen with the board.
		expect(matchingPreset({ ...presetBoard('console'), theme: 'light' })).toBe('console');
		expect(matchingPreset({ ...presetBoard('console'), scale: 1.5 })).toBe('console');
	});

	it('does not care about block order', () => {
		const shuffled: RoomBoard = {
			...presetBoard('wall'),
			blocks: ['qr', 'statement', 'map', 'question']
		};
		expect(matchingPreset(shuffled)).toBe('wall');
	});

	it('applies a template over the room rather than instead of it', () => {
		const room: RoomBoard = { ...presetBoard('console'), theme: 'light', scale: 1.5 };
		const lobby = applyPreset(room, 'lobby');
		expect(lobby.blocks).toEqual(presetBoard('lobby').blocks);
		expect(lobby.sizes).toEqual(presetBoard('lobby').sizes);
		expect(lobby.theme).toBe('light');
		expect(lobby.scale).toBe(1.5);
	});

	it('lets a template that names a lighting have it', () => {
		const room: RoomBoard = { ...presetBoard('console'), theme: 'light' };
		expect(applyPreset(room, 'kiosk').theme).toBe('dark');
	});
});
