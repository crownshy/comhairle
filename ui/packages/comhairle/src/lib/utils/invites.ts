import type { ConversationDto, EventDto, InviteDto, UserDto } from '@crownshy/api-client/api';

/**
 * Generates an invite link for a conversation or event.
 */
export function inviteUrl(
	url: URL,
	invite: InviteDto,
	conversation: ConversationDto,
	event?: EventDto
) {
	return `${url.origin}/conversations/${conversation.id}/${event ? `events/${event.id}/` : ''}invite/${invite.id}`;
}

export function matchCurrentUserAgainstInvite(user: UserDto, invite: InviteDto): boolean {
	if (!user.email) return false;
	if (typeof invite.inviteType !== 'string' && !('email' in invite.inviteType)) return false;

	if (typeof invite.inviteType !== 'string' && user.email === invite.inviteType.email)
		return true;

	return false;
}
