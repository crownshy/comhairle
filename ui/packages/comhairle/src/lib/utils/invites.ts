import type { ConversationDto, EventDto, InviteDto, UserDto } from '@crownshy/api-client/api';

/**
 * Generates an invite link for a conversation or event.
 */
export function inviteUrl(
	url: URL,
	inviteId: InviteDto['id'],
	conversationId: ConversationDto['id'],
	eventId?: EventDto['id']
) {
	return `${url.origin}/conversations/${conversationId}/${eventId ? `events/${eventId}/` : ''}invite/${inviteId}`;
}

export function embedInviteUrl(
	url: URL,
	invite: InviteDto['id'],
	conversation: ConversationDto['id'],
	event?: EventDto['id']
) {
	return inviteUrl(url, invite, conversation, event) + '?embed=true';
}

export function matchCurrentUserAgainstInvite(user: UserDto, invite: InviteDto): boolean {
	if (!user.email) return false;
	if (typeof invite.inviteType !== 'string' && !('email' in invite.inviteType)) return false;

	if (typeof invite.inviteType !== 'string' && user.email === invite.inviteType.email)
		return true;

	return false;
}
