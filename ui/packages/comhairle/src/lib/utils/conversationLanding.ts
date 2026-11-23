import { resolve } from '$app/paths';
import type { ConversationDto } from '@crownshy/api-client/api';
import { conversationSections } from '$lib/config/conversation-steps';
import { conversationStatus, type ConversationStatus } from './conversationStatus';

export type LandingConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

const FALLBACK_SECTION_PATH = 'configure';

const LANDING_SECTION_PATH: Record<ConversationStatus, string> = {
	draft: 'configure',
	live: 'monitor',
	completed: 'report'
};

/** Where opening a conversation from the admin takes you, picked from its status. */
export function conversationLandingUrl(conversation: LandingConversation): string {
	const base = resolve('/(admin)/admin/conversations/[conversation_id]', {
		conversation_id: conversation.id
	});
	return `${base}/${landingSectionPath(conversation)}`;
}

// A completed conversation that never went live would land on a section locked until
// launch, so it opens Configure instead.
function landingSectionPath(conversation: LandingConversation): string {
	const path = LANDING_SECTION_PATH[conversationStatus(conversation)];
	const section = conversationSections.find((candidate) => candidate.path === path);
	if (section?.requiresLive && !conversation.isLive) return FALLBACK_SECTION_PATH;
	return path;
}
