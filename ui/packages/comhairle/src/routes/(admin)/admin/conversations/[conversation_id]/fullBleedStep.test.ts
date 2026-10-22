import { describe, expect, it } from 'vitest';
import { isFullBleedStepPage, stepIdFromPath } from './fullBleedStep';

const setup = '/admin/conversations/c1/design/step/s1/setup';

describe('isFullBleedStepPage', () => {
	it('is true only for an editor on the HeyForm Setup tab', () => {
		expect(isFullBleedStepPage(setup, 'heyform', true)).toBe(true);
		expect(isFullBleedStepPage(`${setup}/`, 'heyform', true)).toBe(true);
		expect(isFullBleedStepPage(setup, 'heyform', false)).toBe(false);
		expect(isFullBleedStepPage(setup, 'polis', true)).toBe(false);
		expect(isFullBleedStepPage(setup.replace('setup', 'configure'), 'heyform', true)).toBe(
			false
		);
	});
});

describe('stepIdFromPath', () => {
	it('reads the step id from a step path', () => {
		expect(stepIdFromPath(setup)).toBe('s1');
		expect(stepIdFromPath('/admin/conversations/c1/design')).toBeUndefined();
	});
});
