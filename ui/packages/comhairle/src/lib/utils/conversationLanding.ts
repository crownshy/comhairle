import { resolve } from '$app/paths';
import type { ConversationDto } from '@crownshy/api-client/api';
import { conversationSections } from '$lib/config/conversation-steps';
import Conversation, { type ConversationStatus } from '$lib/interfaces/Conversation';
import { MODERATION_SECTIONS, type ConversationSurface } from './conversationSurface';

export type LandingConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

const FALLBACK_SECTION_PATH = 'configure';

const LANDING_SECTION_PATH: Record<ConversationStatus, string> = {
	draft: 'configure',
	live: 'monitor/overview',
	completed: 'report'
};

/**
 * Where opening a conversation from the admin takes you: picked from its status, or Moderate
 * for someone on the moderation surface.
 */
export function conversationLandingUrl(
	conversation: LandingConversation,
	surface: ConversationSurface = 'full'
): string {
	const base = resolve('/(admin)/admin/conversations/[conversation_id]', {
		conversation_id: conversation.id
	});
	const section =
		surface === 'moderation' ? MODERATION_SECTIONS[0] : landingSectionPath(conversation);
	return `${base}/${section}`;
}

// A completed conversation that never went live would land on a section locked until
// launch, so it opens Configure instead.
function landingSectionPath(conversation: LandingConversation): string {
	const path = LANDING_SECTION_PATH[Conversation.getStatus(conversation)];
	const section = conversationSections.find((candidate) => candidate.path === path);
	if (section?.requiresLive && !conversation.isLive) return FALLBACK_SECTION_PATH;
	return path;
}
