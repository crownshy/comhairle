import { describe, it, expect } from 'vitest';
import type { UserOrganizationAccess } from '@crownshy/api-client/api';
import { currentHostingOrganizationName, hostingOrganizationOptions } from './hostingOrganizations';

function access(id: string, name: string, canUpdate: boolean): UserOrganizationAccess {
	return {
		canDelete: false,
		canManageTeam: false,
		canUpdate,
		isAssociated: true,
		organization: { id, name }
	} as UserOrganizationAccess;
}

const ORGANIZATIONS = [
	access('b', 'Zebra Council', true),
	access('a', 'Acorn Trust', true),
	access('c', 'Member Only', false)
];

describe('hostingOrganizationOptions', () => {
	it('keeps only organisations the user can update, sorted by name', () => {
		expect(hostingOrganizationOptions(ORGANIZATIONS)).toEqual([
			{ id: 'a', name: 'Acorn Trust' },
			{ id: 'b', name: 'Zebra Council' }
		]);
	});
});

describe('currentHostingOrganizationName', () => {
	it('is null when no organisation hosts the conversation', () => {
		expect(currentHostingOrganizationName(ORGANIZATIONS, null)).toBeNull();
	});

	it('finds the name even when the user cannot update that organisation', () => {
		expect(currentHostingOrganizationName(ORGANIZATIONS, 'c')).toBe('Member Only');
	});

	it('falls back when the user does not belong to the host', () => {
		expect(currentHostingOrganizationName(ORGANIZATIONS, 'x')).toBe('Another organisation');
	});
});
