import type { ConversationDto, InviteDto } from '@crownshy/api-client/api';

/**
 * Generates an invite link for a conversation or event.
 */
export function inviteUrl(url: URL, invite: InviteDto, conversation: ConversationDto) {
	return `${url.origin}/conversations/${conversation.slug ?? conversation.id}/invite/${invite.id}`;
}
