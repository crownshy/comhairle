import { describe, expect, it } from 'vitest';
import { nextStep } from './nextStep';

describe('nextStep', () => {
	it.each([
		['draft', 'Continue setting up', '/admin/conversations/abc/configure'],
		['live', 'Monitor responses', '/admin/conversations/abc/monitor'],
		['completed', 'Write the report', '/admin/conversations/abc/report']
	] as const)('sends a %s conversation to its next section', (status, label, href) => {
		expect(nextStep('abc', status)).toEqual({ label, href });
	});
});
