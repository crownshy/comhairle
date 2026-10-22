import type { EmailType } from '@crownshy/api-client/api';

type EmailTypeCopy = {
	name: string;
	sentWhen: string;
	/** What the base version covers, shown where a conversation title would be. */
	allScopeLabel: string;
};

export const EMAIL_TYPE_COPY: Record<EmailType, EmailTypeCopy> = {
	conversation_invite: {
		name: 'Conversation invite',
		sentWhen: 'Sent when you invite people to take part in a conversation.',
		allScopeLabel: 'All conversations'
	},
	event_registration_invite: {
		name: 'Event invite',
		sentWhen: 'Sent when you invite people to register for an event.',
		allScopeLabel: 'All events'
	},
	event_registration_confirmation: {
		name: 'Event registration confirmation',
		sentWhen: 'Sent straight after someone registers for an event.',
		allScopeLabel: 'All events'
	},
	event_reminder: {
		name: 'Event reminder',
		sentWhen: 'Sent shortly before an event someone registered for.',
		allScopeLabel: 'All events'
	}
};
