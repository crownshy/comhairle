/**
 * Cross-navigation signal naming a conversation that was just created, so the page it
 * lands on can make its newness obvious (focus + select the auto-generated title, ready
 * to rename) instead of looking identical to the conversation the user came from.
 *
 * {@link import('$lib/components/NewConversationButton.svelte')} flags the new id here
 * right before navigating; the configure page reads it on mount, acts if it matches its
 * own conversation, then clears it so a later manual visit does not re-trigger.
 */
let id = $state<string | null>(null);

export const justCreatedConversation = {
	/** The id of the just-created conversation, or `null` when nothing is pending. */
	get id() {
		return id;
	},
	/** Flag a freshly-created conversation for its landing page to acknowledge. */
	flag(conversationId: string) {
		id = conversationId;
	},
	/** Clear the flag once the landing page has acknowledged it. */
	clear() {
		id = null;
	}
};
