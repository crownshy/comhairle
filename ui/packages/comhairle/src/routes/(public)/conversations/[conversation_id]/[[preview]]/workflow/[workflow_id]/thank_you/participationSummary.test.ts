import { describe, expect, it } from 'vitest';
import { EARLY_PARTICIPANT_LIMIT, participationSummary } from './participationSummary';

describe('participationSummary', () => {
	it('marks early participants instead of showing a small count', () => {
		expect(participationSummary(0, 'en')).toEqual({ kind: 'early' });
		expect(participationSummary(EARLY_PARTICIPANT_LIMIT - 1, 'en')).toEqual({ kind: 'early' });
	});

	it('shows the count from fifteen people on', () => {
		expect(participationSummary(EARLY_PARTICIPANT_LIMIT, 'en')).toEqual({
			kind: 'count',
			formattedCount: '15'
		});
	});

	it('formats the count for the reader locale', () => {
		expect(participationSummary(1234, 'en')).toEqual({
			kind: 'count',
			formattedCount: '1,234'
		});
		expect(participationSummary(1234, 'fr')).toEqual({
			kind: 'count',
			formattedCount: (1234).toLocaleString('fr')
		});
	});
});
