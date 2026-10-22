import type { OptionDto } from '@crownshy/api-client/api';
import { describe, expect, it } from 'vitest';
import { formatAnswer } from './formatAnswer';

const options: OptionDto[] = [
	{ id: 'a', label: 'Woman', position: 0 },
	{ id: 'b', label: 'Man', position: 1 }
];

describe('formatAnswer', () => {
	it('shows plain numbers, dates and text as they are', () => {
		expect(formatAnswer({ question_type: 'number', options: [] }, 25)).toBe('25');
		expect(formatAnswer({ question_type: 'date', options: [] }, '2024/02')).toBe('2024/02');
		expect(formatAnswer({ question_type: 'any_string', options: [] }, 'Irish')).toBe('Irish');
	});

	it('shows selected option labels with other text and prefer not to say', () => {
		const version = { question_type: 'multi_select' as const, options };
		expect(formatAnswer(version, { option_ids: ['a', 'b'] })).toBe('Woman, Man');
		expect(formatAnswer(version, { option_ids: ['a'], other_text: ' Retired ' })).toBe(
			'Woman, Other: Retired'
		);
		expect(formatAnswer(version, { option_ids: [], prefer_not_to_say: true })).toBe(
			'Prefer not to say'
		);
	});

	it('shows each kind of location', () => {
		const version = { question_type: 'location' as const, options: [] };
		expect(formatAnswer(version, { postcode: 'SW1A 1AA' })).toBe('SW1A 1AA');
		expect(formatAnswer(version, { country: 'Ireland' })).toBe('Ireland');
		expect(formatAnswer(version, { latitude: 1.5, longitude: 2.5 })).toBe('1.5, 2.5');
	});

	it('falls back to JSON for custom answers', () => {
		expect(formatAnswer({ question_type: 'custom', options: [] }, { a: 1 })).toBe('{"a":1}');
	});
});
