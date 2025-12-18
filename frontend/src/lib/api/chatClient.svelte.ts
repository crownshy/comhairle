import { browser } from '$app/environment';

export interface MessageReference {
	id: string;
	content: string;
	dataset_id: string;
	document_id: string;
	document_name: string;
}

export interface ChatSessionMessage {
	id?: string;
	content: string;
	role: string;
	reference?: MessageReference[];
}

export interface ChatSession {
	id: string;
	chat_id: string;
	name?: string;
	messages?: ChatSessionMessage[];
}

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
	session = $state<ChatSession | null>(null);

	private chatId: string;
	private baseUrl: string;
	private abortController: AbortController | null = null;

	constructor(chatId: string, baseUrl = '/api') {
		this.chatId = chatId;
		this.baseUrl = baseUrl;
	}

	async getSession(sessionId: string): Promise<ChatSession | null> {
		try {
			const response = await fetch(
				`${this.baseUrl}/bot/chats/${this.chatId}/sessions/${sessionId}`,
				{
					method: 'GET',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include'
				}
			);

			if (!response.ok) {
				this.error = `Failed to get session: ${response.statusText}`;
				return null;
			}

			const session = await response.json();
			this.session = session;
			return session;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to get session';
			return null;
		}
	}

	async createSession(name: string): Promise<ChatSession | null> {
		try {
			const response = await fetch(
				`${this.baseUrl}/bot/chats/${this.chatId}/sessions`,
				{
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include',
					body: JSON.stringify({ name })
				}
			);

			if (!response.ok) {
				this.error = `Failed to create session: ${response.statusText}`;
				return null;
			}

			const session = await response.json();
			this.session = session;
			return session;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to create session';
			return null;
		}
	}

	async send(question: string): Promise<string> {
		if (!browser) return '';

		if (!this.session?.id) {
			this.error = 'No session ID. Call createSession() first.';
			return '';
		}

		this.abort();
		this.isStreaming = true;
		this.currentAnswer = '';
		this.currentReference = null;
		this.error = null;
		this.abortController = new AbortController();

		try {
			const response = await fetch(
				`${this.baseUrl}/bot/chats/${this.chatId}/sessions/${this.session.id}`,
				{
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include',
					body: JSON.stringify({
						question,
						stream: true
					}),
					signal: this.abortController.signal
				}
			);

			if (!response.ok) {
				const text = await response.text();
				try {
					const errorData = JSON.parse(text);
					this.error = errorData.message || `Request failed: ${response.statusText}`;
				} catch {
					this.error = text || `Request failed: ${response.statusText}`;
				}
				this.isStreaming = false;
				return '';
			}

			const reader = response.body?.getReader();
			if (!reader) {
				this.error = 'No response body';
				this.isStreaming = false;
				return '';
			}

			const decoder = new TextDecoder('utf-8');
			let buffer = '';

			while (true) {
				const { done, value } = await reader.read();
				if (done) break;

				buffer += decoder.decode(value, { stream: true });
				
				const lines = buffer.split('\n');
				buffer = lines.pop() || ''; // Keep incomplete line in buffer

				for (const line of lines) {
					if (line.startsWith('data:')) {
						try {
							const jsonStr = line.replace('data:', '').trim();
							const json = JSON.parse(jsonStr);

							// Update with the answer from the stream
							if (json.data?.answer) {
								this.currentAnswer = json.data.answer;
							}
							if (json.data?.reference) {
								this.currentReference = json.data.reference;
							}
						} catch (e) {
							console.warn('Failed to parse SSE chunk:', line);
						}
					}
				}
			}

			// Process any remaining buffer
			if (buffer.trim()) {
				if (buffer.startsWith('data:')) {
					try {
						const jsonStr = buffer.replace('data:', '').trim();
						const json = JSON.parse(jsonStr);
						if (json.data?.answer) {
							this.currentAnswer = json.data.answer;
						}
						if (json.data?.reference) {
							this.currentReference = json.data.reference;
						}
					} catch (e) {
						console.warn('Failed to parse final chunk');
					}
				}
			}
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
	}
}