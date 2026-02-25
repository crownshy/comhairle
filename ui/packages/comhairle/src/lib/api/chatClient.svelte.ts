import { browser } from '$app/environment';
import type { ComhairleChatSession } from '@crown-shy/api-client/api';
import { apiClient } from '@crown-shy/api-client/client';

export interface ReferenceChunk {
	id: string;
	content: string;
	document_id: string;
	document_name: string;
	dataset_id: string;
	image_id?: string;
	positions?: number[][];
	url?: string | null;
	similarity?: number;
	vector_similarity?: number;
	term_similarity?: number;
	doc_type?: string[];
}

export interface ChatReference {
	total: number;
	chunks: ReferenceChunk[];
	doc_aggs?: { doc_name: string; doc_id: string; count: number }[];
}

export class ChatClient {
	currentAnswer = $state('');
	currentReference = $state<ChatReference | null>(null);
	error = $state<string | null>(null);
	isStreaming = $state(false);
	session = $state<ComhairleChatSession | null>(null);
	botServiceSessionId = $state<string | null>(null);

	private conversationId?: string;
	private baseUrl: string;
	private abortController: AbortController | null = null;

	constructor(conversationId?: string, baseUrl = '/api') {
		this.conversationId = conversationId;
		this.baseUrl = baseUrl;
	}

	async getSession(): Promise<ComhairleChatSession | null> {
		try {
			if (!this.conversationId) throw new Error('Missing conversation id');

			const session = await apiClient.GetChatSessionHistory({
				params: { conversation_id: this.conversationId }
			});

			this.session = session;
			return session;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to get session';
			return null;
		}
	}

	private parseSSELine(line: string): void {
		if (!line.startsWith('data:')) return;

		try {
			const jsonStr = line.replace('data:', '').trim();
			const json = JSON.parse(jsonStr);

			if (json.data?.answer) {
				this.currentAnswer = json.data.answer;
			}
			if (json.data?.reference) {
				this.currentReference = json.data.reference;
			}
		} catch {
			console.warn('Failed to parse SSE chunk:', line);
		}
	}

	private async readStream(reader: ReadableStreamDefaultReader<Uint8Array>): Promise<void> {
		const decoder = new TextDecoder('utf-8');
		let buffer = '';

		while (true) {
			const { done, value } = await reader.read();
			if (done) break;

			buffer += decoder.decode(value, { stream: true });
			const lines = buffer.split('\n');
			buffer = lines.pop() || '';

			for (const line of lines) {
				this.parseSSELine(line);
			}
		}

		if (buffer.trim()) {
			this.parseSSELine(buffer);
		}
	}

	private async handleErrorResponse(response: Response): Promise<void> {
		const text = await response.text();
		try {
			const errorData = JSON.parse(text);
			this.error = errorData.message || `Request failed: ${response.statusText}`;
		} catch {
			this.error = text || `Request failed: ${response.statusText}`;
		}
	}

	private resetStreamState(): void {
		this.abort();
		this.isStreaming = true;
		this.currentAnswer = '';
		this.currentReference = null;
		this.error = null;
		this.abortController = new AbortController();
	}

	async send(question: string): Promise<string> {
		if (!browser) return '';

		if (!this.session?.id) {
			this.error = 'No session ID.';
			return '';
		}

		this.resetStreamState();

		try {
			const response = await fetch(
				`${this.baseUrl}/conversation/${this.conversationId}/chat_sessions`,
				{
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include',
					body: JSON.stringify({ question }),
					signal: this.abortController?.signal
				}
			);

			if (!response.ok) {
				await this.handleErrorResponse(response);
				this.isStreaming = false;
				return '';
			}

			const reader = response.body?.getReader();
			if (!reader) {
				this.error = 'No response body';
				this.isStreaming = false;
				return '';
			}

			await this.readStream(reader);
		} catch (e) {
			if (e instanceof Error && e.name !== 'AbortError') {
				this.error = e.message;
			}
		} finally {
			this.isStreaming = false;
			this.abortController = null;
		}

		return this.currentAnswer;
	}

	abort() {
		this.abortController?.abort();
		this.abortController = null;
		this.isStreaming = false;
	}

	reset() {
		this.abort();
		this.currentAnswer = '';
		this.currentReference = null;
		this.error = null;
		this.session = null;
		this.botServiceSessionId = null;
	}
}
