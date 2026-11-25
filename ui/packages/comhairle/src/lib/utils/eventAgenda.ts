import type { BasicEventAgendaItem, EventAgendaItem } from '@crownshy/api-client/api';
import { Minute } from '$lib/utils/units';

const DEFAULT_BREAKOUT_MINUTES = 10;
const DEFAULT_PEOPLE_PER_ROOM = 4;

export type PlenaryDraft = {
	id: string;
	type: 'plenary';
	title: string;
	durationMinutes: number;
	/** The saved item, so its description (not edited anywhere yet) survives a save. */
	source?: BasicEventAgendaItem;
};

export type BreakoutDraft = {
	id: string;
	type: 'breakout';
	prompt: string;
	/** Rich text JSON from the editor. */
	instructions: string;
	durationMinutes: number;
	maxPerRoom: number;
};

/** An agenda item as an editor holds it, on the admin event page or in the call sidebar. */
export type AgendaDraftItem = PlenaryDraft | BreakoutDraft;

export function toAgendaDraft(item: EventAgendaItem): AgendaDraftItem {
	const id = crypto.randomUUID();
	if ('Basic' in item) {
		return {
			id,
			type: 'plenary',
			title: item.Basic.title,
			durationMinutes: item.Basic.estimated_time,
			source: item.Basic
		};
	}
	return {
		id,
		type: 'breakout',
		prompt: item.BreakoutRoom.prompt,
		instructions: item.BreakoutRoom.instructions,
		durationMinutes: item.BreakoutRoom.estimated_time,
		maxPerRoom: item.BreakoutRoom.max_per_room ?? DEFAULT_PEOPLE_PER_ROOM
	};
}

export function fromAgendaDraft(draft: AgendaDraftItem): EventAgendaItem {
	const durationMinutes = wholeMinutes(draft.durationMinutes);
	if (draft.type === 'plenary') {
		return {
			Basic: {
				description: draft.source?.description ?? '',
				estimated_time: durationMinutes,
				title: draft.title.trim()
			}
		};
	}
	return {
		BreakoutRoom: {
			instructions: draft.instructions,
			prompt: draft.prompt.trim(),
			estimated_time: durationMinutes,
			time_limit: durationMinutes * 60,
			max_per_room: draft.maxPerRoom
		}
	};
}

export function newPlenaryDraft(): PlenaryDraft {
	return { id: crypto.randomUUID(), type: 'plenary', title: '', durationMinutes: 0 };
}

export function newBreakoutDraft(): BreakoutDraft {
	return {
		id: crypto.randomUUID(),
		type: 'breakout',
		prompt: '',
		instructions: '',
		durationMinutes: DEFAULT_BREAKOUT_MINUTES,
		maxPerRoom: DEFAULT_PEOPLE_PER_ROOM
	};
}

/** What a list row shows as the item's name. A breakout is named by its question. */
export function agendaDraftTitle(draft: AgendaDraftItem): string {
	return (draft.type === 'plenary' ? draft.title : draft.prompt).trim();
}

export type AgendaDraftProblem = {
	field: 'title' | 'minutes' | 'people';
	/** Says what to do, ready to show under the field. */
	message: string;
};

/**
 * What stops an item saving from the call sidebar, or null. A plenary item needs a title;
 * a breakout needs at least a minute and two people per room.
 */
export function agendaDraftProblem(draft: AgendaDraftItem): AgendaDraftProblem | null {
	if (draft.type === 'plenary') {
		return draft.title.trim() === '' ? { field: 'title', message: 'Add a title' } : null;
	}
	if (!(draft.durationMinutes >= 1)) {
		return { field: 'minutes', message: 'Give it at least 1 minute' };
	}
	if (!(draft.maxPerRoom >= 2)) {
		return { field: 'people', message: 'Rooms need at least 2 people' };
	}
	return null;
}

/** The items after the current step: the only ones a host can change during a call. */
export function upcomingAgendaItems(
	items: EventAgendaItem[],
	currentStep: number
): EventAgendaItem[] {
	return items.slice(currentStep + 1);
}

/** Returns a copy of `items` with the item at `from` moved to `to`. */
export function moveAgendaDraft<T>(items: T[], from: number, to: number): T[] {
	if (to < 0 || to >= items.length) return items;
	const moved = [...items];
	const [item] = moved.splice(from, 1);
	moved.splice(to, 0, item);
	return moved;
}

export type AgendaSchedule = {
	/** When each item starts, in agenda order. */
	starts: Date[];
	end: Date;
	totalMinutes: number;
};

/** Lays the agenda out back to back from the event's start time. */
export function agendaSchedule(drafts: AgendaDraftItem[], eventStart: Date): AgendaSchedule {
	let elapsedMinutes = 0;
	const starts = drafts.map((draft) => {
		const start = new Date(eventStart.getTime() + elapsedMinutes * Minute);
		elapsedMinutes += wholeMinutes(draft.durationMinutes);
		return start;
	});
	return {
		starts,
		end: new Date(eventStart.getTime() + elapsedMinutes * Minute),
		totalMinutes: elapsedMinutes
	};
}

/** "45 min", "1h", "1h 25m". */
export function formatAgendaMinutes(minutes: number): string {
	if (minutes < 60) return `${minutes} min`;
	const hours = Math.floor(minutes / 60);
	const rest = minutes % 60;
	return rest === 0 ? `${hours}h` : `${hours}h ${rest}m`;
}

// A cleared number input binds as null, and typed values can be fractional or negative.
function wholeMinutes(value: number | null | undefined): number {
	return Math.max(0, Math.round(value ?? 0));
}
