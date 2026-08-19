import { describe, it, expect } from 'vitest';
import { mapApiAgenda } from './liveEventAgenda';

describe('mapApiAgenda', () => {
	it('maps a basic item to a plenary agenda item', () => {
		expect(
			mapApiAgenda([{ Basic: { title: 'Welcome', description: 'Intro', estimated_time: 5 } }])
		).toEqual([{ id: '1', title: 'Welcome', type: 'plenary' }]);
	});

	it('maps a breakout item, titling it from the prompt', () => {
		expect(
			mapApiAgenda([
				{
					BreakoutRoom: {
						prompt: 'What matters most to you?',
						instructions: 'Talk it through',
						estimated_time: 15,
						max_per_room: 4
					}
				}
			])
		).toEqual([
			{
				id: '1',
				title: 'What matters most to you?',
				type: 'breakout',
				breakoutQuestion: 'What matters most to you?',
				breakoutDescription: 'Talk it through',
				durationMinutes: 15,
				maxPerRoom: 4
			}
		]);
	});

	it('falls back to a generic title when a breakout has no prompt', () => {
		const [item] = mapApiAgenda([
			{ BreakoutRoom: { prompt: '', instructions: '', estimated_time: 10 } }
		]);

		expect(item.title).toBe('Breakout session');
	});

	it('leaves maxPerRoom undefined when the API sends null', () => {
		const [item] = mapApiAgenda([
			{
				BreakoutRoom: {
					prompt: 'Q',
					instructions: '',
					estimated_time: 10,
					max_per_room: null
				}
			}
		]);

		expect(item.maxPerRoom).toBeUndefined();
	});

	it('numbers ids from one, in agenda order', () => {
		const items = mapApiAgenda([
			{ Basic: { title: 'One', description: '', estimated_time: 5 } },
			{ BreakoutRoom: { prompt: 'Two', instructions: '', estimated_time: 5 } },
			{ Basic: { title: 'Three', description: '', estimated_time: 5 } }
		]);

		expect(items.map((i) => i.id)).toEqual(['1', '2', '3']);
	});

	it('maps an empty agenda to an empty list', () => {
		expect(mapApiAgenda([])).toEqual([]);
	});
});
