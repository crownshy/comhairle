import { ChatClient, type ChatReference } from './chatClient.svelte';

/**
 * Unified chat message shape shared across all chat surfaces (ChatBot, LearningAssistant, etc.).
 */
export type ChatMessage = {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	reference: ChatReference | null;
	timestamp: Date | null;
	streaming?: boolean;
	error?: string | null;
};

/**
 * Reactive chat session. One instance per conversation id, cached in a module-level map.
 * Owns a single ChatClient and the canonical message list so multiple UI components
 * (ChatBot, LearningAssistant, etc.) stay in sync without duplicating state or API calls.
 */
export class ChatSession {
	messages = $state<ChatMessage[]>([]);
	error = $state<string | null>(null);
	initializing = $state(false);
	initialized = $state(false);

	readonly conversationId: string;
	private client: ChatClient;
	private initPromise: Promise<void> | null = null;
	private disposeMirror: (() => void) | null = null;

	constructor(conversationId: string) {
		this.conversationId = conversationId;
		this.client = new ChatClient(conversationId);
	}

	get isStreaming(): boolean {
		return this.client.isStreaming;
	}

	/**
	 * Load prior conversation history. Safe to call multiple times — only runs once.
	 */
	async init(): Promise<void> {
		if (this.initPromise) return this.initPromise;
		this.initPromise = (async () => {
			try {
				this.initializing = true;
				this.error = null;
				const session = await this.client.getSession();
				if (!session) {
					this.error = this.client.error || 'Failed to load session';
					return;
				}
				if (session.messages?.length) {
					this.messages = session.messages.map((msg, idx) => ({
						id: msg.id ? `${msg.id}-${msg.role}` : `msg-${idx}`,
						role: msg.role === 'assistant' ? 'assistant' : 'user',
						content: msg.content,
						reference: msg.reference?.length
							? {
									total: msg.reference.length,
									chunks: msg.reference.map((ref) => ({
										id: ref.id,
										content: ref.content,
										document_id: ref.document_id,
										document_name: ref.document_name,
										dataset_id: ref.dataset_id
									}))
								}
							: null,
						timestamp: null
					}));
				}
				this.initialized = true;
			} catch (e) {
				this.error = e instanceof Error ? e.message : 'Failed to load session';
			} finally {
				this.initializing = false;
			}
		})();
		return this.initPromise;
	}

	/**
	 * Force a fresh init attempt after a previous failure.
	 */
	async retryInit(): Promise<void> {
		this.initPromise = null;
		this.initialized = false;
		this.error = null;
		return this.init();
	}

	/**
	 * Clear any current error state. Does not affect message-level errors.
	 */
	clearError(): void {
		this.error = null;
	}

	/**
	 * Send a question. Appends a user message and a streaming assistant placeholder,
	 * mirrors stream chunks into the placeholder, then finalizes on completion.
	 *
	 * @param question Clean question text shown in the user message bubble.
	 * @param llmQuestion Optional alternative string actually sent to the LLM
	 *   (e.g. with extra page context). Defaults to `question`.
	 */
	async send(question: string, llmQuestion?: string): Promise<void> {
		if (!this.initialized || this.isStreaming) return;

		// Per-send failures live on the assistant ChatMessage; clear any stale
		// session-level error so a previous transient failure doesn't keep
		// blocking the transcript UI.
		this.error = null;

		const now = Date.now();
		const userMsg: ChatMessage = {
			id: `user-${now}`,
			role: 'user',
			content: question,
			reference: null,
			timestamp: new Date()
		};
		const botId = `bot-${now}`;
		const botMsg: ChatMessage = {
			id: botId,
			role: 'assistant',
			content: '',
			reference: null,
			timestamp: null,
			streaming: true
		};
		this.messages = [...this.messages, userMsg, botMsg];

		// Mirror streaming chunks from the client into the assistant placeholder.
		const dispose = $effect.root(() => {
			$effect(() => {
				const answer = this.client.currentAnswer;
				const reference = this.client.currentReference;
				const idx = this.messages.findIndex((m) => m.id === botId);
				if (idx === -1) return;
				const current = this.messages[idx];
				if (current.content === answer && current.reference === reference) return;
				const next = [...this.messages];
				next[idx] = { ...current, content: answer, reference };
				this.messages = next;
			});
		});
		this.disposeMirror = dispose;

		let sendError: string | null = null;
		try {
			await this.client.send(llmQuestion ?? question);
			if (this.client.error) sendError = this.client.error;
		} catch (e) {
			sendError = e instanceof Error ? e.message : 'Failed to get answer';
		} finally {
			dispose();
			this.disposeMirror = null;
		}

		// Finalize the assistant message with whatever the client ended up with.
		const idx = this.messages.findIndex((m) => m.id === botId);
		if (idx !== -1) {
			const next = [...this.messages];
			next[idx] = {
				...next[idx],
				content: this.client.currentAnswer || next[idx].content,
				reference: this.client.currentReference ?? next[idx].reference,
				timestamp: new Date(),
				streaming: false,
				error: sendError
			};
			this.messages = next;
		}
	}

	/**
	 * Retry the last user question if its assistant reply errored. Removes the
	 * failed assistant message and resends the question.
	 */
	async retryLast(): Promise<void> {
		if (this.isStreaming) return;
		const msgs = this.messages;
		let lastUserIdx = -1;
		for (let i = msgs.length - 1; i >= 0; i--) {
			if (msgs[i].role === 'user') {
				lastUserIdx = i;
				break;
			}
		}
		if (lastUserIdx === -1) return;
		const question = msgs[lastUserIdx].content;

		this.messages = msgs.slice(0, lastUserIdx);
		this.error = null;
		await this.send(question);
	}

	/**
	 * Abort any in-flight stream and detach the mirror effect.
	 */
	abort(): void {
		this.disposeMirror?.();
		this.disposeMirror = null;
		this.client.abort();
	}
}

const sessionCache = new Map<string, ChatSession>();
const MAX_CACHED_SESSIONS = 20;

/**
 * Evict the least-recently-used session, aborting any in-flight stream.
 * Map iteration order is insertion order, and `getChatSession` re-inserts on
 * access, so the first entry is the oldest unused one.
 */
function evictOldestChatSession(): void {
	const oldest = sessionCache.keys().next();
	if (oldest.done) return;
	const id = oldest.value;
	sessionCache.get(id)?.abort();
	sessionCache.delete(id);
}

/**
 * Drop a cached ChatSession and abort any in-flight work. Returns true if a
 * session was present and removed.
 */
export function clearChatSession(conversationId: string): boolean {
	const session = sessionCache.get(conversationId);
	if (!session) return false;
	session.abort();
	return sessionCache.delete(conversationId);
}

/**
 * Get (or lazily create) the shared ChatSession for a conversation id. Multiple
 * components passing the same id receive the exact same instance, so they share
 * messages, streaming state, and errors.
 *
 * The cache is capped at MAX_CACHED_SESSIONS with LRU eviction so a long-lived
 * SPA session that visits many conversations doesn't retain ChatClients and
 * message history indefinitely.
 */
export function getChatSession(conversationId: string): ChatSession {
	const existing = sessionCache.get(conversationId);
	if (existing) {
		// Re-insert to mark as most-recently-used.
		sessionCache.delete(conversationId);
		sessionCache.set(conversationId, existing);
		return existing;
	}
	if (sessionCache.size >= MAX_CACHED_SESSIONS) {
		evictOldestChatSession();
	}
	const session = new ChatSession(conversationId);
	sessionCache.set(conversationId, session);
	return session;
}
