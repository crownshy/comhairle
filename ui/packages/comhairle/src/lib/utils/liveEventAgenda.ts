import type { EventAgendaItem } from '@crownshy/api-client/api';
import type { AgendaItem } from '$lib/components/LiveEvent/types';

/** Map API agenda items to the shape the live event UI renders. */
export function mapApiAgenda(items: EventAgendaItem[]): AgendaItem[] {
	return items.map((item, index) => {
		if ('Basic' in item) {
			return {
				id: String(index + 1),
				title: item.Basic.title,
				type: 'plenary' as const
			};
		} else {
			return {
				id: String(index + 1),
				title: item.BreakoutRoom.prompt || 'Breakout session',
				type: 'breakout' as const,
				breakoutQuestion: item.BreakoutRoom.prompt,
				breakoutDescription: item.BreakoutRoom.instructions,
				durationMinutes: item.BreakoutRoom.estimated_time,
				maxPerRoom: item.BreakoutRoom.max_per_room ?? undefined
			};
		}
	});
}
