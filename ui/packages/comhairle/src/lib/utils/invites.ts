import type { InviteDto, UserDto } from '@crownshy/api-client/api';

/**
 * Generates an invite link for a conversation or event.
 */
export function inviteUrl(url: URL, inviteId: string, conversationId: string, eventId?: string) {
	return `${url.origin}/conversations/${conversationId}/${eventId ? `events/${eventId}/` : ''}invite/${inviteId}`;
}

export function embedInviteUrl(
	url: URL,
	inviteId: string,
	conversationId: string,
	eventId?: string
) {
	return inviteUrl(url, inviteId, conversationId, eventId) + '?embed=true';
}

export function matchCurrentUserAgainstInvite(user: UserDto, invite: InviteDto): boolean {
	if (!user.email) return false;
	if (typeof invite.inviteType !== 'string' && !('email' in invite.inviteType)) return false;

	if (typeof invite.inviteType !== 'string' && user.email === invite.inviteType.email)
		return true;

	return false;
}
