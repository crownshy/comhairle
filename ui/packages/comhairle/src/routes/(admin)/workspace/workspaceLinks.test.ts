import { describe, expect, it } from 'vitest';
import { workspaceCrumbs, workspaceLinks } from './workspaceLinks';

const labels = (roles: { isSiteAdmin: boolean; isSuperAdmin: boolean }) =>
	workspaceLinks(roles).map(({ label }) => label);

describe('workspaceLinks', () => {
	it('shows site admin and super admin sections only to those roles', () => {
		expect(labels({ isSiteAdmin: false, isSuperAdmin: false })).toEqual([
			'Conversations',
			'Organisations',
			'Handbook'
		]);
		expect(labels({ isSiteAdmin: true, isSuperAdmin: true })).toEqual([
			'Conversations',
			'Media library',
			'Email templates',
			'Organisations',
			'Regions',
			'System roles',
			'Handbook'
		]);
	});
});

describe('workspaceCrumbs', () => {
	it('is empty on the landing page and outside known sections', () => {
		expect(workspaceCrumbs('/admin')).toEqual([]);
		expect(workspaceCrumbs('/admin/')).toEqual([]);
		expect(workspaceCrumbs('/admin/unknown')).toEqual([]);
	});

	it('names the section without a link when you are on its list', () => {
		expect(workspaceCrumbs('/admin/media-library')).toEqual([{ label: 'Media library' }]);
	});

	it('links the section from pages below it', () => {
		expect(workspaceCrumbs('/admin/organizations/abc')).toEqual([
			{ label: 'Organisations', href: '/admin/organizations' }
		]);
		expect(workspaceCrumbs('/admin/email-template-configs/new')).toEqual([
			{ label: 'Email templates', href: '/admin/email-template-configs' },
			{ label: 'New' }
		]);
	});
});
