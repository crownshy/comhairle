import { describe, it, expect } from 'vitest';
import type { EmailTemplateConfigWithScopeDto, EmailTypeSchema } from '@crownshy/api-client/api';
import { buildEmailTypeView, buildEmailTypeGroups } from './emailTemplateGroups';

const schema = (email_type: EmailTypeSchema['email_type'], default_subject: string) =>
	({ email_type, default_subject, slots: [], template: '', variables: [] }) as EmailTypeSchema;

const SCHEMAS = [
	schema('conversation_invite', 'Invitation to take part'),
	schema('event_reminder', 'Upcoming event reminder')
];

const config = (
	overrides: Partial<EmailTemplateConfigWithScopeDto>
): EmailTemplateConfigWithScopeDto =>
	({
		id: 'id',
		emailType: 'conversation_invite',
		ownerId: 'owner',
		slots: { type: 'conversation_invite' },
		createdAt: '2026-10-21T10:00:00Z',
		...overrides
	}) as EmailTemplateConfigWithScopeDto;

describe('buildEmailTypeGroups', () => {
	it('shows every email type, with the default when nothing is customised', () => {
		const groups = buildEmailTypeGroups(SCHEMAS, []);

		expect(groups.map((group) => group.emailType)).toEqual([
			'conversation_invite',
			'event_reminder'
		]);
		expect(groups[1].rows).toEqual([
			{ key: 'event_reminder-default', source: 'default', subject: 'Upcoming event reminder' }
		]);
	});

	it('puts the all-conversations version first, then conversations by title', () => {
		const groups = buildEmailTypeGroups(SCHEMAS, [
			config({ id: 'b', conversationId: 'c2', conversationTitle: 'Zebra crossings' }),
			config({ id: 'all', subject: 'Join us' }),
			config({ id: 'a', conversationId: 'c1', conversationTitle: 'Allotments' })
		]);

		expect(groups[0].rows.map((row) => [row.key, row.source])).toEqual([
			['all', 'all'],
			['a', 'conversation'],
			['b', 'conversation']
		]);
		expect(groups[0].rows[0].subject).toBe('Join us');
	});

	it('keeps the default as the base row when only conversations are customised', () => {
		const groups = buildEmailTypeGroups(SCHEMAS, [
			config({ id: 'a', conversationId: 'c1', conversationTitle: 'Allotments' })
		]);

		expect(groups[0].rows[0].source).toBe('default');
		expect(groups[0].rows[1].subject).toBe('Invitation to take part');
	});
});

describe('buildEmailTypeView', () => {
	const CONVERSATIONS = [
		{ id: 'c1', title: 'Allotments' },
		{ id: 'c2', title: 'Zebra crossings' },
		{ id: 'c3', title: 'Bus routes' }
	];

	it('splits the default from conversations with their own version', () => {
		const view = buildEmailTypeView(
			SCHEMAS[0],
			[
				config({ id: 'all' }),
				config({ id: 'a', conversationId: 'c1', conversationTitle: 'Allotments' })
			],
			CONVERSATIONS
		);

		expect(view.base.key).toBe('all');
		expect(view.conversationVersions.map((row) => row.key)).toEqual(['a']);
	});

	it('lists the conversations that fall back to the default', () => {
		const view = buildEmailTypeView(
			SCHEMAS[0],
			[config({ id: 'a', conversationId: 'c1', conversationTitle: 'Allotments' })],
			CONVERSATIONS
		);

		expect(view.usingBase.map((conversation) => conversation.id)).toEqual(['c2', 'c3']);
	});

	it('has no fallback list for emails that cannot be per conversation', () => {
		expect(buildEmailTypeView(SCHEMAS[1], [], CONVERSATIONS).usingBase).toEqual([]);
	});
});
