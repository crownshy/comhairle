import { describe, expect, it } from 'vitest';
import { isLinkActive, workspaceGroups } from './workspaceLinks';

const labels = (roles: { isSiteAdmin: boolean; isSuperAdmin: boolean }) =>
	workspaceGroups(roles).map((group) => [group.label, group.links.map(({ label }) => label)]);

describe('workspaceGroups', () => {
	it('shows site admin and super admin sections only to those roles', () => {
		expect(labels({ isSiteAdmin: false, isSuperAdmin: false })).toEqual([
			[undefined, ['Conversations']],
			['Administration', ['Organisations']]
		]);
		expect(labels({ isSiteAdmin: true, isSuperAdmin: true })).toEqual([
			[undefined, ['Conversations']],
			[
				'Library',
				[
					'Media library',
					'Email templates',
					'Demographic forms',
					'Hybrid works (prototype)'
				]
			],
			['Administration', ['Organisations', 'Regions', 'System roles']]
		]);
	});
});

describe('isLinkActive', () => {
	const organisations = workspaceGroups({ isSiteAdmin: false, isSuperAdmin: false })[1].links[0];

	it('matches the section and the pages below it', () => {
		expect(isLinkActive(organisations, '/admin/organizations')).toBe(true);
		expect(isLinkActive(organisations, '/admin/organizations/abc')).toBe(true);
	});

	it('does not match a section that only shares a prefix', () => {
		expect(isLinkActive(organisations, '/admin/organizations-old')).toBe(false);
		expect(isLinkActive(organisations, '/admin')).toBe(false);
	});
});
