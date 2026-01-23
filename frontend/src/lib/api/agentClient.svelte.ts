import { browser } from '$app/environment';

export interface AgentMessageReference {
	id: string;
	content: string;
	dataset_id: string;
	document_id: string;
	document_name: string;
}

export interface AgentSessionMessage {
	id?: string;
	content: string;
	role: string;
	reference?: AgentMessageReference[];
}

export interface AgentSession {
	id: string;
	agent_id: string;
	messages: AgentSessionMessage[];
	dsl?: unknown;
}

export interface BotServiceUserSession {
	id: string;
	user_id: string;
	conversation_id: string;
	bot_service_session_id: string;
	workflow_step_id?: string;
}

export interface ExtractedClaim {
	id: string;
	content: string;
}

export class AgentClient {
	currentAnswer = $state('');
	error = $state<string | null>(null);
	isStreaming = $state(false);
	session = $state<AgentSession | null>(null);
	botServiceSession = $state<BotServiceUserSession | null>(null);
	extractedClaims = $state<ExtractedClaim[]>([]);

	private isParsingOpinion = false;
	private currentOpinionContent = '';
	private opinionMarker = '<br>\n\nopinion:\n\n';

	private agentId: string;
	private conversationId: string;
	private workflowId: string;
	private workflowStepId: string;
	private baseUrl: string;
	private abortController: AbortController | null = null;

	constructor(
		agentId: string,
		conversationId: string,
		workflowId: string,
		workflowStepId: string,
		baseUrl = '/api'
	) {
		this.agentId = agentId;
		this.conversationId = conversationId;
		this.workflowId = workflowId;
		this.workflowStepId = workflowStepId;
		this.baseUrl = baseUrl;
	}

	/**
	 * Get or create a bot service user session for this workflow step.
	 * This links the user to an agent session in the bot service.
	 */
	async getOrCreateWorkflowStepSession(): Promise<BotServiceUserSession | null> {
		try {
			const response = await fetch(
				`${this.baseUrl}/conversation/${this.conversationId}/workflow/${this.workflowId}/workflow_step/${this.workflowStepId}/bot_service_session`,
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

			const data = await response.json();
			this.botServiceSession = data;

			// Now fetch the full agent session
			return data;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to get or create session';
			return null;
		}
	}

	/**
	 * Create a new agent session
	 */
	async createNewAgentSession(): Promise<AgentSession | null> {
		try {
			const response = await fetch(
				`${this.baseUrl}/bot/agents/${this.agentId}/sessions/`,
				{
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include'
				}
			);

			if (!response.ok) {
				this.error = `Failed to create agent session: ${response.statusText}`;
				return null;
			}

			const session = await response.json();
			this.session = session;

			this.botServiceSession = {
				id: session.id,
				user_id: '',
				conversation_id: this.conversationId,
				bot_service_session_id: session.id,
				workflow_step_id: this.workflowStepId
			};
			return session;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to create agent session';
			return null;
		}
	}

	/**
	 * Get the agent session details including message history
	 */
	async getAgentSession(sessionId: string): Promise<AgentSession | null> {
		try {
			const response = await fetch(
				`${this.baseUrl}/bot/agents/${this.agentId}/sessions/${sessionId}`,
				{
					method: 'GET',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include'
				}
			);

			if (!response.ok) {
				this.error = `Failed to get agent session: ${response.statusText}`;
				return null;
			}

			const session = await response.json();
			this.session = session;
			return session;
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to get agent session';
			return null;
		}
	}

	/**
	 * Initialize the client by getting the workflow step session and agent session
	 */
	async initialize(): Promise<boolean> {
		const botSession = await this.getOrCreateWorkflowStepSession();
		if (!botSession) {
			return false;
		}

		const agentSession = await this.getAgentSession(botSession.bot_service_session_id);
		return agentSession !== null;
	}

	/**
	 * Initialize the client with a fresh session (no history persistence)
	 */
	async initializeFresh(): Promise<boolean> {
		const agentSession = await this.createNewAgentSession();
		return agentSession !== null;
	}

	private parseSSELine(line: string): void {
		if (!line.startsWith('data:')) return;

		const jsonStr = line.replace('data:', '').trim();
		
		if (jsonStr === '[DONE]') {
			return;
		}

		try {
			const json = JSON.parse(jsonStr);
			console.debug('Agent SSE event:', json.event, json.data?.component_type, json.data?.outputs);

			if (json.data?.answer) {
				this.currentAnswer = json.data.answer;
				return;
			}

			if (json.event === 'message_end') {
				if (this.isParsingOpinion && this.currentOpinionContent.trim()) {
					const claimId = `claim-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
					this.extractedClaims = [...this.extractedClaims, {
						id: claimId,
						content: this.currentOpinionContent.trim()
					}];
				}
				this.isParsingOpinion = false;
				this.currentOpinionContent = '';
				return;
			}

			if (json.event === 'node_finished' && json.data) {
				const { component_type, outputs } = json.data;
				
				if (component_type === 'Message' && outputs?.content) {
					const content = Array.isArray(outputs.content) 
						? outputs.content.join('') 
						: outputs.content;
					if (content && typeof content === 'string') {
						this.currentAnswer = content;
					}
				}
				
				if (component_type === 'Agent' && outputs?.content) {
					const content = outputs.content;
					if (content && typeof content === 'string') {
						this.currentAnswer = content;
					}
				}
			}
			
			if (json.event === 'message' && json.data?.content) {
				const content = json.data.content as string;
				
				if (content.includes('<br>') && content.includes('opinion:')) {
					this.isParsingOpinion = true;
					this.currentOpinionContent = '';
					return;
				}
				
				if (this.isParsingOpinion) {
					this.currentOpinionContent += content;
				} else {
					this.currentAnswer += content;
				}
			}
		} catch {
			if (jsonStr) {
				console.warn('Failed to parse SSE chunk:', line);
			}
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
		this.error = null;
		this.abortController = new AbortController();
	}

	async send(question: string): Promise<string> {
		if (!browser) return '';

		if (!this.botServiceSession?.bot_service_session_id) {
			this.error = 'No session ID. Call initialize() first.';
			return '';
		}

		this.resetStreamState();

		try {
			const response = await fetch(
				`${this.baseUrl}/bot/agents/${this.agentId}/sessions/${this.botServiceSession.bot_service_session_id}`,
				{
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					credentials: 'include',
					body: JSON.stringify({ question, workflow_step_id: this.workflowStepId }),
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
		this.error = null;
		this.session = null;
		this.botServiceSession = null;
	}
}
