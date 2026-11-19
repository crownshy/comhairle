import { describe, expect, it } from 'vitest';
import { toGroupSummaries } from './groupSummaries';

describe('toGroupSummaries', () => {
	it('keys groups by number, whether the id is stored as a number or a string', () => {
		expect(
			toGroupSummaries([
				{ groupId: 0, name: 'Commuters', description: 'Want more buses' },
				{ groupId: '1', name: 'Drivers', description: 'Want more parking' }
			])
		).toEqual({
			0: { name: 'Commuters', summary: 'Want more buses' },
			1: { name: 'Drivers', summary: 'Want more parking' }
		});
	});

	it('skips a group without both a name and a description', () => {
		expect(
			toGroupSummaries([
				{ groupId: 0, name: 'Commuters' },
				{ groupId: 1, name: null, description: 'Want more parking' }
			])
		).toEqual({});
	});
});
