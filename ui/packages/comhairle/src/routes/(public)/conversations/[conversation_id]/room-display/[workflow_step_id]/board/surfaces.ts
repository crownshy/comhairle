/**
 * Keeps the wall and console windows in step over a `BroadcastChannel` per step, on one
 * machine. A window that opens late says hello to get the current state. See ADR-0045.
 */

import { isRoomBoard, type RoomBoard } from './blocks';

/** Which half of the console layout this window shows. `both` shows the two on one page. */
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

/** Fixed window names, so opening a surface twice focuses the existing window. */
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
	/** A new window wants the current state. Only the console window answers, so replies do not race. */
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
 * Opens the channel and says hello. Without `BroadcastChannel` (server render, old kiosk
 * browsers) sends are dropped and the one-page layout still works.
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
		// A stale tab on an older build can send shapes this one does not know, so validate.
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
