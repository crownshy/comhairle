import { describe, it, expect } from 'vitest';
import type { EventAgendaItem } from '@crownshy/api-client/api';
import {
	agendaDraftTitle,
	agendaSchedule,
	formatAgendaMinutes,
	fromAgendaDraft,
	moveAgendaDraft,
	newBreakoutDraft,
	newPlenaryDraft,
	toAgendaDraft
} from './eventAgenda';

const welcome: EventAgendaItem = {
	Basic: { title: 'Welcome', description: 'Say hello', estimated_time: 5 }
};
const breakout: EventAgendaItem = {
	BreakoutRoom: {
		prompt: 'What matters most?',
		instructions: '{"type":"doc"}',
		estimated_time: 15,
		time_limit: 900,
		max_per_room: 5
	}
};

describe('agenda drafts', () => {
	it('round-trips saved items, keeping the description nobody edits', () => {
		expect(fromAgendaDraft(toAgendaDraft(welcome))).toEqual(welcome);
		expect(fromAgendaDraft(toAgendaDraft(breakout))).toEqual(breakout);
	});

	it('applies edits to a breakout', () => {
		const draft = toAgendaDraft(breakout);
		if (draft.type !== 'breakout') throw new Error('expected a breakout draft');

		const edited = fromAgendaDraft({ ...draft, prompt: ' New question ', durationMinutes: 20 });

		expect(edited).toEqual({
			BreakoutRoom: {
				prompt: 'New question',
				instructions: '{"type":"doc"}',
				estimated_time: 20,
				time_limit: 1200,
				max_per_room: 5
			}
		});
	});

	it('saves a cleared or fractional duration as whole minutes', () => {
		const cleared = { ...newPlenaryDraft(), durationMinutes: null as unknown as number };
		expect(fromAgendaDraft(cleared)).toMatchObject({ Basic: { estimated_time: 0 } });
		expect(fromAgendaDraft({ ...newPlenaryDraft(), durationMinutes: 7.6 })).toMatchObject({
			Basic: { estimated_time: 8 }
		});
	});

	it('builds new items with empty text and default breakout settings', () => {
		expect(fromAgendaDraft(newPlenaryDraft())).toEqual({
			Basic: { title: '', description: '', estimated_time: 0 }
		});
		expect(fromAgendaDraft(newBreakoutDraft())).toEqual({
			BreakoutRoom: {
				prompt: '',
				instructions: '',
				estimated_time: 10,
				time_limit: 600,
				max_per_room: 4
			}
		});
	});

	it('names a breakout by its question', () => {
		expect(agendaDraftTitle({ ...newBreakoutDraft(), prompt: ' Why? ' })).toBe('Why?');
		expect(agendaDraftTitle({ ...newPlenaryDraft(), title: 'Welcome' })).toBe('Welcome');
	});
});

describe('moveAgendaDraft', () => {
	it('moves an item up or down', () => {
		expect(moveAgendaDraft(['a', 'b', 'c'], 2, 1)).toEqual(['a', 'c', 'b']);
		expect(moveAgendaDraft(['a', 'b', 'c'], 0, 1)).toEqual(['b', 'a', 'c']);
	});

	it('ignores a move past either end', () => {
		const items = ['a', 'b'];
		expect(moveAgendaDraft(items, 0, -1)).toBe(items);
		expect(moveAgendaDraft(items, 1, 2)).toBe(items);
	});
});

describe('agendaSchedule', () => {
	const start = new Date('2026-11-20T14:00:00Z');

	it('runs the items back to back from the event start', () => {
		const schedule = agendaSchedule(
			[
				{ ...newPlenaryDraft(), durationMinutes: 10 },
				{ ...newBreakoutDraft(), durationMinutes: 20 },
				{ ...newPlenaryDraft(), durationMinutes: 15 }
			],
			start
		);

		expect(schedule.starts.map((date) => date.toISOString())).toEqual([
			'2026-11-20T14:00:00.000Z',
			'2026-11-20T14:10:00.000Z',
			'2026-11-20T14:30:00.000Z'
		]);
		expect(schedule.end.toISOString()).toBe('2026-11-20T14:45:00.000Z');
		expect(schedule.totalMinutes).toBe(45);
	});

	it('ends at the start when the agenda is empty', () => {
		expect(agendaSchedule([], start)).toEqual({ starts: [], end: start, totalMinutes: 0 });
	});
});

describe('formatAgendaMinutes', () => {
	it('uses minutes under an hour and hours above', () => {
		expect(formatAgendaMinutes(0)).toBe('0 min');
		expect(formatAgendaMinutes(45)).toBe('45 min');
		expect(formatAgendaMinutes(60)).toBe('1h');
		expect(formatAgendaMinutes(85)).toBe('1h 25m');
	});
});
