import type { Snippet } from 'svelte';

export const CONVERSATION_TAB_EXTRAS_CTX = Symbol('conversation-tab-extras');

export type ConversationTabExtras = {
	primary: Snippet | null;
	secondary: Snippet | null;
};
