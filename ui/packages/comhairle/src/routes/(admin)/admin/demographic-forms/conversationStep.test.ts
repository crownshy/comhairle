import { describe, expect, it } from 'vitest';
import { createInitialForms } from './demographicPrototypeData';
import {
	askedCount,
	createStep,
	overrideCount,
	pinnableVersions,
	requiredCount,
	resetToForm,
	selectableForms,
	setAsk,
	versionLabel
} from './conversationStep';

describe('conversation step helpers', () => {
	const forms = createInitialForms();
	const form = forms[0];
	const settings = {
		name: 'About you',
		policy: 'latest' as const,
		pinnedVersion: null,
		allowReturn: true,
		mustComplete: true
	};

	it('only offers published forms', () => {
		expect(selectableForms(forms).map((f) => f.id)).not.toContain('f-youth');
	});

	it('lists pinnable versions newest first', () => {
		expect(pinnableVersions(form)).toEqual([2, 1]);
	});

	it('starts with every question asked and the form defaults', () => {
		const step = createStep('s1', form, settings);
		expect(askedCount(step)).toBe(form.questions.length);
		expect(requiredCount(step)).toBe(form.questions.filter((q) => q.required).length);
		expect(overrideCount(step, form)).toBe(0);
	});

	it('labels the version policy', () => {
		const latest = createStep('s1', form, settings);
		expect(versionLabel(latest, form)).toBe(`Follows latest (v${form.version})`);
		const pinned = createStep('s2', form, { ...settings, policy: 'pinned', pinnedVersion: 1 });
		expect(versionLabel(pinned, form)).toBe('Pinned to v1');
	});

	it('counts overrides and clears required when a question is turned off', () => {
		const step = createStep('s1', form, settings);
		const requiredIndex = step.questions.findIndex((q) => q.required);
		step.questions[requiredIndex] = setAsk(step.questions[requiredIndex], false);
		expect(step.questions[requiredIndex].required).toBe(false);
		expect(overrideCount(step, form)).toBe(1);
		expect(overrideCount(resetToForm(step, form), form)).toBe(0);
	});
});
