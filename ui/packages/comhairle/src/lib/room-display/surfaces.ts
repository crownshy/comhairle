/**
 * The two surfaces of the console layout, and how they stay in step when they are two
 * browser windows rather than two halves of one page.
 *
 * Real use is one laptop driving one projector. The facilitator hovers a statement on
 * the laptop and the wall shows it; picks a group and the wall lists what that group
 * thinks. On one page that is one component's state. In two windows it has to travel,
 * and this module is the wire.
 *
 * The wire is a `BroadcastChannel` named for the step, which reaches every window of
 * this origin on the same machine and nothing else. That is the whole of the two-window
 * case (a projector is a second screen on the laptop, not a second computer), and it
 * costs no server. Two machines would need a socket and are not this module's problem.
 *
 * What travels: the console state (which statement is focused, what the wall's main
 * area shows) and the board itself, so a size or a block flipped on the laptop lands on
 * the wall. A window that opens late says hello and whoever holds a console answers
 * with both, so the wall can be opened after the console has been set up.
 */

import { isRoomBoard, type RoomBoard } from './blocks';

/**
 * Which half of the console layout a window is. `both` is the one-page prototype,
 * where the pair can be judged in one screenshot; `wall` and `console` are the two
 * windows of a real room.
 */
export type RoomSurface = 'both' | 'wall' | 'console';

export function parseSurface(raw: string | null | undefined): RoomSurface {
	return raw === 'wall' || raw === 'console' ? raw : 'both';
}

/** The same board, on the other surface. `both` drops the parameter rather than spelling it. */
export function surfaceHref(current: string | URL, surface: RoomSurface): string {
	const url = new URL(current);
	if (surface === 'both') url.searchParams.delete('surface');
	else url.searchParams.set('surface', surface);
	return url.href;
}

/**
 * Window names, so opening the console twice focuses the window that is already
 * there rather than stacking a second one.
 */
export const SURFACE_WINDOW_NAMES: Record<Exclude<RoomSurface, 'both'>, string> = {
	wall: 'comhairle-room-wall',
	console: 'comhairle-room-console'
};

/** What the wall's main area shows. The map unless the facilitator asks for a list. */
export type WallView = { kind: 'map' } | { kind: 'group'; groupId: number } | { kind: 'consensus' };

/** Everything the console decides and the wall has to follow. */
export interface ConsoleState {
	focusedTid: number | null;
	wallView: WallView;
}

export const INITIAL_CONSOLE_STATE: ConsoleState = { focusedTid: null, wallView: { kind: 'map' } };

function isWallView(value: unknown): value is WallView {
	if (typeof value !== 'object' || value === null) return false;
	const view = value as { kind?: unknown; groupId?: unknown };
	if (view.kind === 'map' || view.kind === 'consensus') return true;
	return view.kind === 'group' && typeof view.groupId === 'number';
}

export function isConsoleState(value: unknown): value is ConsoleState {
	if (typeof value !== 'object' || value === null) return false;
	const state = value as { focusedTid?: unknown; wallView?: unknown };
	const tidOk = state.focusedTid === null || typeof state.focusedTid === 'number';
	return tidOk && isWallView(state.wallView);
}

type Message =
	| { type: 'hello' }
	| { type: 'state'; state: ConsoleState }
	| { type: 'board'; board: RoomBoard };

export interface SurfaceLinkHandlers {
	onState: (state: ConsoleState) => void;
	onBoard: (board: RoomBoard) => void;
	/**
	 * A window has just opened and wants the current state and board. Answer by
	 * sending both, or do nothing if this window holds no console: a wall has nothing
	 * to say, and two answers would race.
	 */
	onHello: () => void;
}

export interface SurfaceLink {
	sendState: (state: ConsoleState) => void;
	sendBoard: (board: RoomBoard) => void;
	close: () => void;
}

export function surfaceChannelName(workflowStepId: string): string {
	return `comhairle:room-display:${workflowStepId}`;
}

/**
 * Opens the wire and says hello. Without `BroadcastChannel` (server render, an old
 * kiosk browser) this is a link to nowhere: sends are dropped and nothing arrives,
 * which leaves the one-page layout working exactly as it did.
 */
export function openSurfaceLink(
	workflowStepId: string,
	handlers: SurfaceLinkHandlers
): SurfaceLink {
	if (typeof BroadcastChannel === 'undefined') {
		return { sendState: () => {}, sendBoard: () => {}, close: () => {} };
	}
	const channel = new BroadcastChannel(surfaceChannelName(workflowStepId));
	const post = (message: Message) => channel.postMessage(message);

	channel.onmessage = (event: MessageEvent<unknown>) => {
		const message = event.data as Partial<Message> | null;
		if (typeof message !== 'object' || message === null) return;
		// Same origin only, but a stale tab on an older build could still send a shape
		// this one does not know. Dropped, not applied.
		if (message.type === 'hello') handlers.onHello();
		else if (message.type === 'state' && isConsoleState(message.state)) {
			handlers.onState(message.state);
		} else if (message.type === 'board' && isRoomBoard(message.board)) {
			handlers.onBoard(message.board);
		}
	};
	post({ type: 'hello' });

	return {
		sendState: (state) => post({ type: 'state', state }),
		sendBoard: (board) => post({ type: 'board', board }),
		close: () => channel.close()
	};
}
