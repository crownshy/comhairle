/**
 * Remembers the last board per machine in `localStorage`, so a rebooted projector comes
 * back to it. Storage can throw or be empty, so every path falls back to "nothing stored".
 */

import { isRoomBoard, type RoomBoard } from './blocks';

const STORAGE_KEY = 'comhairle:room-display:board';

export function readStoredBoard(): RoomBoard | null {
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw === null) return null;
		const parsed: unknown = JSON.parse(raw);
		// Discard anything from an older version or edited by hand rather than half-apply it.
		return isRoomBoard(parsed) ? parsed : null;
	} catch {
		return null;
	}
}

export function writeStoredBoard(board: RoomBoard): void {
	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(board));
	} catch {
		// Storage is full or blocked. The board still works for this session.
	}
}
