import { describe, expect, it } from 'vitest';
import { moderatePracticeRows, practiceFeedback, practiceRows } from './practiceStatements';

describe('practiceRows', () => {
	it('starts every statement waiting, numbered from 1', () => {
		const rows = practiceRows();
		expect(rows.map((row) => row.moderation_status)).toEqual(['pending', 'pending', 'pending']);
		expect(rows.map((row) => row.polis_statement_id)).toEqual([1, 2, 3]);
	});
});

describe('moderatePracticeRows', () => {
	it('keeps the reason on a reject and clears it otherwise', () => {
		const rejected = moderatePracticeRows(
			practiceRows(),
			['practice-2'],
			'rejected',
			'Privacy'
		);
		expect(rejected[1]).toMatchObject({
			moderation_status: 'rejected',
			moderation_reason: 'Privacy'
		});
		expect(rejected[0].moderation_status).toBe('pending');

		const accepted = moderatePracticeRows(rejected, ['practice-2'], 'accepted', 'Privacy');
		expect(accepted[1]).toMatchObject({
			moderation_status: 'accepted',
			moderation_reason: null
		});
	});
});

describe('practiceFeedback', () => {
	it('has nothing to say about waiting statements', () => {
		expect(practiceFeedback(practiceRows())).toEqual([]);
	});

	it('says whether each decision was the right call', () => {
		const rows = moderatePracticeRows(practiceRows(), ['practice-1', 'practice-3'], 'accepted');
		const feedback = practiceFeedback(rows);
		expect(feedback.map(({ number, correct }) => ({ number, correct }))).toEqual([
			{ number: 1, correct: true },
			{ number: 3, correct: false }
		]);
		expect(feedback[0].message).toMatch(/^Right\./);
		expect(feedback[1].message).toMatch(/^Not quite\./);
	});
});
