import { describe, expect, it } from 'vitest';
import { createStarterForm, createStarterQuestions } from './starterSet';

describe('starter set', () => {
	it('has four core questions with unique ids', () => {
		const questions = createStarterQuestions();
		expect(questions).toHaveLength(4);
		expect(new Set(questions.map((q) => q.id)).size).toBe(4);
	});

	it('gives every special category question its own consent wording', () => {
		const special = createStarterQuestions().filter((q) => q.specialCategory);
		expect(special.length).toBeGreaterThan(0);
		for (const q of special) expect(q.consentText.trim()).not.toBe('');
	});

	it('builds a published form that contains exactly those questions', () => {
		const ids = createStarterQuestions().map((q) => q.id);
		const form = createStarterForm();
		expect(form.status).toBe('published');
		expect(form.questions.map((q) => q.questionId)).toEqual(ids);
		expect(form.versions[0].questions).toEqual(form.questions);
	});
});
