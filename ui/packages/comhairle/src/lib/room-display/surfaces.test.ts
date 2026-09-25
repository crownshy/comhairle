import { afterEach, describe, expect, it, vi } from 'vitest';
import { presetBoard } from './blocks';
import {
	INITIAL_CONSOLE_STATE,
	isConsoleState,
	openSurfaceLink,
	parseSurface,
	surfaceChannelName,
	surfaceHref,
	type ConsoleState,
	type SurfaceLink,
	type SurfaceLinkHandlers
} from './surfaces';

describe('parseSurface', () => {
	it('is both unless told otherwise', () => {
		expect(parseSurface(null)).toBe('both');
		expect(parseSurface('')).toBe('both');
		expect(parseSurface('projector')).toBe('both');
		expect(parseSurface('wall')).toBe('wall');
		expect(parseSurface('console')).toBe('console');
	});
});

describe('surfaceHref', () => {
	const here = 'https://example.test/room?mode=demo&scale=1.2';

	it('adds the surface and keeps the board', () => {
		expect(surfaceHref(here, 'console')).toBe(`${here}&surface=console`);
	});

	it('replaces one surface with another', () => {
		expect(surfaceHref(`${here}&surface=wall`, 'console')).toBe(`${here}&surface=console`);
	});

	it('spells both by leaving the parameter out', () => {
		expect(surfaceHref(`${here}&surface=wall`, 'both')).toBe(here);
	});
});

describe('isConsoleState', () => {
	it('accepts each wall view', () => {
		expect(isConsoleState(INITIAL_CONSOLE_STATE)).toBe(true);
		expect(isConsoleState({ focusedTid: 4, wallView: { kind: 'group', groupId: 1 } })).toBe(
			true
		);
		expect(isConsoleState({ focusedTid: null, wallView: { kind: 'consensus' } })).toBe(true);
	});

	it('rejects anything a stale tab might send', () => {
		expect(isConsoleState(null)).toBe(false);
		expect(isConsoleState({ focusedTid: '4', wallView: { kind: 'map' } })).toBe(false);
		expect(isConsoleState({ focusedTid: 4, wallView: { kind: 'group' } })).toBe(false);
		expect(isConsoleState({ focusedTid: 4, wallView: { kind: 'slides' } })).toBe(false);
		expect(isConsoleState({ focusedTid: 4 })).toBe(false);
	});
});

describe('openSurfaceLink', () => {
	const open: SurfaceLink[] = [];
	afterEach(() => {
		for (const link of open.splice(0)) link.close();
	});

	function handlers(): SurfaceLinkHandlers & {
		onState: ReturnType<typeof vi.fn>;
		onBoard: ReturnType<typeof vi.fn>;
		onHello: ReturnType<typeof vi.fn>;
	} {
		return { onState: vi.fn(), onBoard: vi.fn(), onHello: vi.fn() };
	}

	function link(step: string, h: SurfaceLinkHandlers): SurfaceLink {
		const l = openSurfaceLink(step, h);
		open.push(l);
		return l;
	}

	// BroadcastChannel delivers on a later task, so every assertion waits for it.
	const delivered = () => new Promise((resolve) => setTimeout(resolve, 10));

	it('names the channel for the step, so two steps do not hear each other', () => {
		expect(surfaceChannelName('abc')).toBe('comhairle:room-display:abc');
	});

	it('says hello on opening, and only other windows hear it', async () => {
		const wall = handlers();
		link('s1', wall);
		await delivered();
		expect(wall.onHello).not.toHaveBeenCalled();

		const console = handlers();
		link('s1', console);
		await delivered();
		expect(wall.onHello).toHaveBeenCalledTimes(1);
		expect(console.onHello).not.toHaveBeenCalled();
	});

	it('carries the console state to the other window', async () => {
		const wall = handlers();
		link('s2', wall);
		const console = link('s2', handlers());
		await delivered();

		const state: ConsoleState = { focusedTid: 7, wallView: { kind: 'group', groupId: 0 } };
		console.sendState(state);
		await delivered();
		expect(wall.onState).toHaveBeenCalledWith(state);
	});

	it('carries the board', async () => {
		const wall = handlers();
		link('s3', wall);
		const console = link('s3', handlers());
		await delivered();

		const board = { ...presetBoard('console'), scale: 1.5 };
		console.sendBoard(board);
		await delivered();
		expect(wall.onBoard).toHaveBeenCalledWith(board);
	});

	it('does not cross steps', async () => {
		const other = handlers();
		link('s4', other);
		const console = link('s5', handlers());
		await delivered();
		console.sendState(INITIAL_CONSOLE_STATE);
		await delivered();
		expect(other.onHello).not.toHaveBeenCalled();
		expect(other.onState).not.toHaveBeenCalled();
	});

	it('drops a message it does not recognise', async () => {
		const wall = handlers();
		link('s6', wall);
		const raw = new BroadcastChannel(surfaceChannelName('s6'));
		await delivered();
		raw.postMessage({ type: 'state', state: { focusedTid: 'x', wallView: { kind: 'map' } } });
		raw.postMessage({ type: 'board', board: { layout: 'nope' } });
		raw.postMessage('hello');
		raw.postMessage(null);
		await delivered();
		raw.close();
		expect(wall.onState).not.toHaveBeenCalled();
		expect(wall.onBoard).not.toHaveBeenCalled();
	});

	it('hears nothing once closed', async () => {
		const wall = handlers();
		const wallLink = link('s7', wall);
		const console = link('s7', handlers());
		await delivered();
		wallLink.close();
		console.sendState(INITIAL_CONSOLE_STATE);
		await delivered();
		expect(wall.onState).not.toHaveBeenCalled();
	});
});
