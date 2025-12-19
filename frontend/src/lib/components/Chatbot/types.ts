import type { ChatReference } from '$lib/api/chatClient.svelte';

export interface ChatMessage {
	id: string;
	content: string;
	isBot: boolean;
	timestamp: Date;
	reference?: ChatReference | null;
}

export interface InitialQuestion {
	id: string;
	text: string;
	variant?: 'default' | 'primary';
}

export interface ChatBotProps {
	chatId?: string;
	conversationId?: string;
	userId?: string;
	knowledgeBaseIds?: string[];
	title?: string;
	subtitle?: string;
	botName?: string;
	botSubtitle?: string;
	messages?: ChatMessage[];
	placeholder?: string;
	initialQuestions?: InitialQuestion[];
	showInitialQuestions?: boolean;
	onSendMessage?: (message: string) => void;
	onQuestionClick?: (question: string) => void;
}

export type { ChatReference, ReferenceChunk } from '$lib/api/chatClient.svelte';
