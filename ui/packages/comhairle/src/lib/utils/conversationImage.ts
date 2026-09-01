/**
 * Default hero for a Conversation with no banner image of its own.
 *
 * Served from `static/` rather than bundled from `src/lib/assets/` so the path is
 * stable and cacheable, and so a deployment can swap the file without a rebuild.
 */
export const DEFAULT_CONVERSATION_IMAGE = '/hero.webp';

/**
 * The API substitutes its own `default_conversation_image_url` (`api/src/config.rs`)
 * when a Conversation has no media, so `imageUrl` is never empty and a falsy check
 * alone would never fire. Match on the placeholder's filename so any bucket or
 * environment prefix still resolves to our default.
 */
const API_PLACEHOLDER = 'comhairle-conversation-placeholder';

export function conversationImageUrl(imageUrl: string | null | undefined): string {
	const url = imageUrl?.trim();
	return !url || url.includes(API_PLACEHOLDER) ? DEFAULT_CONVERSATION_IMAGE : url;
}
