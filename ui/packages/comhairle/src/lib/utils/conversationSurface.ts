import type { ConversationAction } from '@crownshy/api-client/api';

/**
 * How much of a conversation's admin a user sees. Anyone who can edit the conversation gets
 * all of it. A moderator who can't edit gets only the moderation queue (ADR-0065).
 */
export type ConversationSurface = 'full' | 'moderation';

/** The section paths a moderation surface keeps. */
export const MODERATION_SECTIONS: readonly string[] = ['moderate'];

export function conversationSurface(
	can: (action: ConversationAction) => boolean
): ConversationSurface {
	if (can('conversation_update')) return 'full';
	if (can('conversation_moderate')) return 'moderation';
	return 'full';
}

/** The section path segment of a conversation admin URL, e.g. `moderate`, or null at its root. */
export function sectionSegment(pathname: string, conversationId: string): string | null {
	const base = `/admin/conversations/${conversationId}`;
	if (!pathname.startsWith(`${base}/`)) return null;
	return pathname.slice(base.length + 1).split('/')[0] || null;
}

/**
 * The section to send a user to when the page they opened isn't on their surface, or null
 * when they can stay.
 */
export function surfaceRedirect(
	surface: ConversationSurface,
	segment: string | null
): string | null {
	if (surface === 'full') return null;
	if (segment && MODERATION_SECTIONS.includes(segment)) return null;
	return MODERATION_SECTIONS[0];
}
