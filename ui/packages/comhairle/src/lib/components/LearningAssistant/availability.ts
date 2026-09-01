import type { LocalizedConversationDto } from '@crownshy/api-client/api';

/**
 * The Learning Assistant only answers from parsed knowledge base documents, so it is hidden
 * entirely when the knowledge base is empty. `hasKnowledgeBaseDocs` is the single source for
 * that last part, hoisted to the workflow `+layout.ts` and shared by everything that asks.
 */
export function learningAssistantAvailable(
	conversation: Pick<LocalizedConversationDto, 'chatBotId' | 'enableQaChatBot'> | undefined,
	hasKnowledgeBaseDocs: boolean | undefined
): boolean {
	return !!conversation?.chatBotId && !!conversation.enableQaChatBot && !!hasKnowledgeBaseDocs;
}
