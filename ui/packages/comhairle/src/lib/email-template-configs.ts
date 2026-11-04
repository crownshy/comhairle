import { EmailType } from '@crownshy/api-client/api';

export type EmailScopeResource = 'conversation' | 'event';

export const EMAIL_SCOPE_RESOURCE: Record<EmailType, EmailScopeResource> = {
	conversation_invite: 'conversation',
	event_registration_invite: 'event',
	event_registration_confirmation: 'event',
	event_reminder: 'event'
};

export function isConversationEmail(emailType: EmailType): boolean {
	return EMAIL_SCOPE_RESOURCE[emailType] === 'conversation';
}

export function isEventEmail(emailType: EmailType): boolean {
	return EMAIL_SCOPE_RESOURCE[emailType] === 'event';
}
