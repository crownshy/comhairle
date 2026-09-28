/**
 * What this particular display was last left showing.
 *
 * A projector that reboots, or a tab that crashes two minutes before the room walks
 * in, should come back to the board the facilitator built rather than to the factory
 * default. That state belongs to the machine, not to the conversation, so it lives in
 * `localStorage` rather than anywhere we could read it back from.
 *
 * Which means it is unreliable by construction: private windows, cleared site data
 * and locked-down kiosk profiles all make these throw or come back empty. Every path
 * here degrades to "we remembered nothing", which `resolveBoard` already handles.
 */

import { isRoomBoard, type RoomBoard } from './blocks';

const STORAGE_KEY = 'comhairle:room-display:board';

export function readStoredBoard(): RoomBoard | null {
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw === null) return null;
		const parsed: unknown = JSON.parse(raw);
		// Anything written by an older version, or edited by hand, is discarded rather
		// than half-applied: a board is cheap to rebuild and a broken one is not.
		return isRoomBoard(parsed) ? parsed : null;
	} catch {
		return null;
	}
}

export function writeStoredBoard(board: RoomBoard): void {
	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(board));
	} catch {
		// Storage is full, or blocked. The board still works for this session.
	}
}
