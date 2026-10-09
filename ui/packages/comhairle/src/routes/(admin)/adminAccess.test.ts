import { describe, expect, it } from 'vitest';
import { adminReturnPath } from './adminAccess';
import { canAccessAdminPortal } from '$lib/utils/permissions';

describe('admin portal access', () => {
	it('allows Site Admin and SuperAdmin roles', () => {
		expect(canAccessAdminPortal([{ resource: 'Site', roles: ['Admin'] }])).toBe(true);
		expect(canAccessAdminPortal([{ resource: 'Site', roles: ['SuperAdmin'] }])).toBe(true);
	});

	it('denies participants and resource-scoped admins', () => {
		expect(canAccessAdminPortal([])).toBe(false);
		expect(canAccessAdminPortal(undefined)).toBe(false);
		expect(
			canAccessAdminPortal([{ resource: { Conversation: 'conversation' }, roles: ['Admin'] }])
		).toBe(false);
	});

	it('returns to the public referrer including its query', () => {
		expect(
			adminReturnPath(
				'https://example.com/conversations/test?tab=events',
				new URL('https://example.com/admin')
			)
		).toBe('/conversations/test?tab=events');
	});

	it.each([
		null,
		'invalid',
		'https://another.example/page',
		'https://example.com/admin',
		'https://example.com/admin/conversations',
		'https://example.com//another.example'
	])('uses the home page for unsafe or missing referrers: %s', (referer) => {
		expect(adminReturnPath(referer, new URL('https://example.com/admin'))).toBe('/');
	});
});
