import { describe, expect, it } from 'vitest';
import { roleRecipients } from './roleAssignments';

describe('roleRecipients', () => {
	it('groups and deduplicates direct roles for a user', () => {
		const recipients = roleRecipients(
			[
				{ user_id: 'alice', role_name: 'observer' },
				{ user_id: 'alice', role_name: 'moderator' },
				{ user_id: 'alice', role_name: 'observer' }
			],
			[{ id: 'alice', username: 'Alice', email: 'alice@example.com' }],
			[]
		);
		expect(recipients).toEqual([
			{
				id: 'alice',
				type: 'user',
				kind: 'User',
				name: 'Alice',
				detail: 'alice@example.com',
				roles: ['observer', 'moderator']
			}
		]);
	});

	it('resolves an organization from its membership group, not its resource ID', () => {
		const recipients = roleRecipients(
			[{ group_id: 'membership', role_name: 'content_editor' }],
			[],
			[
				{
					id: 'organization',
					userGroupId: 'membership',
					name: 'Acme',
					contactEmail: 'acme@example.com'
				}
			]
		);
		expect(recipients[0]).toMatchObject({
			id: 'membership',
			type: 'group',
			kind: 'Organization',
			name: 'Acme'
		});
	});

	it('keeps user and group actors distinct even with the same ID', () => {
		const recipients = roleRecipients(
			[
				{ user_id: 'same', role_name: 'observer' },
				{ group_id: 'same', role_name: 'admin' }
			],
			[],
			[{ id: 'organization', userGroupId: 'same', name: 'Acme' }]
		);
		expect(recipients).toHaveLength(2);
		expect(recipients.map((recipient) => recipient.type)).toEqual(
			expect.arrayContaining(['user', 'group'])
		);
	});

	it('hides standalone group grants and ignores missing actors', () => {
		const recipients = roleRecipients(
			[{ group_id: 'group', role_name: 'translator' }, { role_name: 'admin' }],
			[],
			[]
		);
		expect(recipients).toEqual([]);
	});
});
