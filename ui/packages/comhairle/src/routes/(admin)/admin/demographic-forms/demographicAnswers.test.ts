import { describe, expect, it } from 'vitest';
import {
	emptyAnswer,
	isAnswered,
	isChoiceDisabled,
	OTHER_CHOICE,
	PREFER_NOT_TO_SAY,
	summariseAnswer,
	toggleChoice
} from './demographicAnswers';

describe('toggleChoice', () => {
	it('keeps one selection for single choice', () => {
		const first = toggleChoice(emptyAnswer(), 'A', 'single_choice', null);
		expect(toggleChoice(first, 'B', 'single_choice', null).selected).toEqual(['B']);
	});

	it('stops at the maximum for multiple choice', () => {
		let answer = toggleChoice(emptyAnswer(), 'A', 'multiple_choice', 2);
		answer = toggleChoice(answer, 'B', 'multiple_choice', 2);
		expect(isChoiceDisabled(answer, 'C', 'multiple_choice', 2)).toBe(true);
		expect(toggleChoice(answer, 'C', 'multiple_choice', 2).selected).toEqual(['A', 'B']);
	});

	it('makes prefer not to say exclusive and exempt from the maximum', () => {
		let answer = toggleChoice(emptyAnswer(), 'A', 'multiple_choice', 1);
		expect(isChoiceDisabled(answer, PREFER_NOT_TO_SAY, 'multiple_choice', 1)).toBe(false);
		answer = toggleChoice(answer, PREFER_NOT_TO_SAY, 'multiple_choice', 1);
		expect(answer.selected).toEqual([PREFER_NOT_TO_SAY]);
	});
});

describe('test run helpers', () => {
	it('knows when a question is answered', () => {
		expect(isAnswered(emptyAnswer())).toBe(false);
		expect(isAnswered({ selected: ['Woman'], otherText: '', text: '' })).toBe(true);
		expect(isAnswered({ selected: [], otherText: '', text: '  ' })).toBe(false);
		expect(isAnswered({ selected: [], otherText: '', text: '42' })).toBe(true);
	});

	it('summarises answers for the test summary', () => {
		expect(summariseAnswer({ selected: ['Woman'], otherText: '', text: '' })).toBe('Woman');
		expect(summariseAnswer({ selected: [OTHER_CHOICE], otherText: 'Agender', text: '' })).toBe(
			'Other: Agender'
		);
		expect(summariseAnswer(emptyAnswer())).toBe('');
	});
});
