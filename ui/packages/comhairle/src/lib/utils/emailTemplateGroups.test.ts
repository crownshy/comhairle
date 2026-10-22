import { describe, it, expect } from 'vitest';
import type { EmailTemplateConfigWithScopeDto, EmailTypeSchema } from '@crownshy/api-client/api';
import {
	buildEmailTypeGroups,
	filterGroupsForConversation,
	scopedConversations
} from './emailTemplateGroups';

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

describe('filterGroupsForConversation', () => {
	const groups = buildEmailTypeGroups(SCHEMAS, [
		config({ id: 'all' }),
		config({ id: 'a', conversationId: 'c1', conversationTitle: 'Allotments' })
	]);

	it('returns every row when no conversation is chosen', () => {
		expect(filterGroupsForConversation(groups, null)).toBe(groups);
	});

	it('shows the conversation's own version, or what it falls back to', () => {
		const filtered = filterGroupsForConversation(groups, 'c1');

		expect(filtered[0].rows.map((row) => row.key)).toEqual(['a']);
		expect(filtered[1].rows.map((row) => row.source)).toEqual(['default']);
	});
});

describe('scopedConversations', () => {
	it('lists each customised conversation once, by title', () => {
		expect(
			scopedConversations([
				config({ id: '1', conversationId: 'c2', conversationTitle: 'Zebra crossings' }),
				config({ id: '2', conversationId: 'c1', conversationTitle: 'Allotments' }),
				config({
					id: '3',
					conversationId: 'c2',
					conversationTitle: 'Zebra crossings',
					emailType: 'event_reminder'
				}),
				config({ id: '4' })
			])
		).toEqual([
			{ id: 'c1', title: 'Allotments' },
			{ id: 'c2', title: 'Zebra crossings' }
		]);
	});
});
