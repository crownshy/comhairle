import { describe, expect, it } from 'vitest';
import {
	publishImpact,
	createInitialForms,
	createInitialQuestions,
	specialCategoryCount,
	usedInFormsCount
} from './demographicPrototypeData';

describe('demographic prototype data helpers', () => {
	const questions = createInitialQuestions();
	const forms = createInitialForms();

	it('counts the forms a question is used in', () => {
		expect(usedInFormsCount('q-age', forms)).toBe(3);
		expect(usedInFormsCount('q-year-unknown', forms)).toBe(0);
	});

	it('describes who is affected by a publish', () => {
		expect(publishImpact(forms[0])).toBe(
			'3 conversations follow the latest version and will pick up this change. 1 conversation is pinned to v1 and will not change.'
		);
		expect(publishImpact(forms[2])).toContain('nothing else changes');
	});

	it('counts special category questions in a form', () => {
		expect(specialCategoryCount(forms[0], questions)).toBe(0);
		expect(specialCategoryCount(forms[1], questions)).toBe(2);
	});
});
