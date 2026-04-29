import { describe, it, expect } from 'vitest';
import { matchCurrentUserAgainstInvite } from './invites';
import type { InviteDto, UserDto } from '@crownshy/api-client/api';

describe('inviteUtils', () => {
	describe('matchCurrentUserAgainstInvite', () => {
		it('returns true if user email matches invite', () => {
			const user = {
				email: 'foo@bar.com'
			} as UserDto;

			const invite = {
				inviteType: { email: 'foo@bar.com' }
			} as InviteDto;

			const result = matchCurrentUserAgainstInvite(user, invite);

			expect(result).toBe(true);
		});

		it('returns false if user does not have email', () => {
			const user = {
				username: 'foobar'
			} as UserDto;

			const invite = {
				inviteType: { email: 'foo@bar.com' }
			} as InviteDto;

			const result = matchCurrentUserAgainstInvite(user, invite);

			expect(result).toBe(false);
		});

		it('returns false if user email does not match invite email', () => {
			const user = {
				email: 'bar@foo.com'
			} as UserDto;

			const invite = {
				inviteType: { email: 'foo@bar.com' }
			} as InviteDto;

			const result = matchCurrentUserAgainstInvite(user, invite);

			expect(result).toBe(false);
		});

		it('returns false if user inviteType is wrong', () => {
			const user = {
				email: 'bar@foo.com'
			} as UserDto;

			const invite = {
				inviteType: 'open'
			} as InviteDto;

			const result = matchCurrentUserAgainstInvite(user, invite);

			expect(result).toBe(false);
		});
	});
});
