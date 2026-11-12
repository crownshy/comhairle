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
		expect(formatAnswer({ question_type: 'text', options: [] }, 'Irish')).toBe('Irish');
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
});
