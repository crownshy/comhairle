import { describe, expect, it } from 'vitest';
import { pluralise } from './pluralise';

describe('pluralise', () => {
	it('keeps the noun singular for one', () => {
		expect(pluralise(1, 'statement')).toBe('statement');
	});

	it('adds an s for zero and for more than one', () => {
		expect(pluralise(0, 'statement')).toBe('statements');
		expect(pluralise(3, 'statement')).toBe('statements');
	});
});
