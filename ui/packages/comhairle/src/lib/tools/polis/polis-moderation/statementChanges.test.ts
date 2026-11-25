import { describe, expect, it } from 'vitest';
import { isUnchanged, statementChanges } from './statementChanges';

describe('statementChanges', () => {
	it('marks every word kept when nothing changed', () => {
		const parts = statementChanges('Fund more buses', 'Fund more buses');
		expect(parts.every((part) => part.kind === 'kept')).toBe(true);
	});

	it('marks removed and added words', () => {
		const parts = statementChanges('Fund more buses and trains', 'Fund more buses');
		expect(
			parts.filter((part) => part.kind === 'removed').map((part) => part.text.trim())
		).toEqual(['and trains']);
		expect(parts.some((part) => part.kind === 'added')).toBe(false);
	});

	it('ignores spaces around the text', () => {
		expect(isUnchanged('  Fund more buses ', 'Fund more buses')).toBe(true);
		expect(isUnchanged('Fund more buses', 'Fund more trains')).toBe(false);
	});
});
