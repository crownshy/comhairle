import { describe, expect, it } from 'vitest';
import { getOrganizationSections } from './organizationSections';

describe('getOrganizationSections', () => {
	it('returns a single organizations section from backend entries', () => {
		const sections = getOrganizationSections({
			organizations: [
				{
					organization: { id: 'org-1', name: 'Org 1' } as never
				},
				{
					organization: { id: 'org-2', name: 'Org 2' } as never
				}
			]
		});

		expect(sections.map((section) => section.key)).toEqual(['organizations']);
		expect(sections[0]?.organizations).toEqual([
			{ id: 'org-1', name: 'Org 1' },
			{ id: 'org-2', name: 'Org 2' }
		]);
	});
});
