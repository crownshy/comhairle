import { browser } from '$app/environment';
import { notifications } from '$lib/notifications.svelte';
import type { ComhairleAgentSession } from './api';
import { apiClient } from './client';

export interface ParsedSessionMessage {
	id: string;
	content: string;
	isBot: boolean;
	timestamp: Date | null;
}

export interface ParsedSessionClaim {
	id: string;
	content: string;
	status: 'pending' | 'approved' | 'editing' | 'streaming';
}

export interface ParsedSessionHistory {
	messages: ParsedSessionMessage[];
}

export function stripOpinionPrefix(content: string): string {
	const patterns = [
		/^<br>\s*\n*\s*opinion:\s*\n*/i,
		/^<br>\s*opinion:\s*/i,
		/^\n*opinion:\s*\n*/i
	];

	let result = content;
	for (const pattern of patterns) {
		result = result.replace(pattern, '');
	}

	return result.trim();
}

export function stripOpinionMarker(content: string): string {
	const patterns = [/<br>\s*opinion:/i, /<br>\n+opinion:\n*/i, /\n+opinion:/i];

	for (const pattern of patterns) {
		const match = content.match(pattern);
		if (match && match.index !== undefined) {
			return content.substring(0, match.index).trim();
		}
	}

	return content;
}

export function parseSessionHistory(
	session: ComhairleAgentSession,
	_topicName: string
): ParsedSessionHistory {
	const messages: ParsedSessionMessage[] = [];

	if (session.messages && session.messages.length > 0) {
		for (let i = 0; i < session.messages.length; i++) {
			const msg = session.messages[i];
			const isBot = msg.role === 'assistant';
			let content = msg.content;

			const uniqueId = `${msg.role}-${i}-${msg.id || Date.now()}`;

			if (isBot) {
				content = stripOpinionMarker(content);
			}

			messages.push({
				id: uniqueId,
				content,
				isBot,
				timestamp: null
			});
		}
	}

	return { messages };
}

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
	extractedClaims = $state<ExtractedClaim[]>([]);
	streamingClaim = $state<ExtractedClaim | null>(null);

	private isParsingOpinion = false;
	private sawOpinionMarker = false;
	private currentOpinionContent = '';
	private streamingClaimId = '';
	private opinionMarker = '<br>\n\nopinion:\n\n';

	private workflowStepId: string;
	private baseUrl: string;
	private abortController: AbortController | null = null;

	onClaimUpdate?: (
		streamingClaim: ExtractedClaim | null,
		extractedClaims: ExtractedClaim[]
	) => void;

	constructor(workflowStepId: string, baseUrl = '/api') {
		this.workflowStepId = workflowStepId;
		this.baseUrl = baseUrl;
	}

	async getSessionHistory(): Promise<ComhairleAgentSession | null> {
		try {
			const session = await apiClient.GetElicitationBotSessionHistory({
				params: {
					workflow_step_id: this.workflowStepId
				}
			});

			return session;
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Error retrieving session history ' });
			return null;
		}
	}

	private parseSSELine(line: string): void {
		if (!line.startsWith('data:')) return;

		const jsonStr = line.replace('data:', '').trim();

		if (jsonStr === '[DONE]') {
			return;
		}

		try {
			const json = JSON.parse(jsonStr);

			if (json.data?.answer) {
				this.currentAnswer = json.data.answer;
				return;
			}

			if (json.event === 'message_end') {
				if (this.isParsingOpinion && this.currentOpinionContent.trim()) {
					this.extractedClaims = [
						...this.extractedClaims,
						{
							id: this.streamingClaimId,
							content: this.currentOpinionContent.trim()
						}
					];
				}
				this.isParsingOpinion = false;
				this.currentOpinionContent = '';
				this.streamingClaim = null;
				this.streamingClaimId = '';
				this.onClaimUpdate?.(null, this.extractedClaims);
				return;
			}

			if (json.event === 'node_finished' && json.data) {
				if (this.sawOpinionMarker) return;

				const { component_type, outputs } = json.data;

				if (component_type === 'Message' && outputs?.content) {
					const content = Array.isArray(outputs.content)
						? outputs.content.join('')
						: outputs.content;
					if (content && typeof content === 'string') {
						const stripped = stripOpinionMarker(content);
						if (stripped) this.currentAnswer = stripped;
					}
				}

				if (component_type === 'Agent' && outputs?.content) {
					const content = outputs.content;
					if (content && typeof content === 'string') {
						const stripped = stripOpinionMarker(content);
						if (stripped) this.currentAnswer = stripped;
					}
				}
			}

			if (json.event === 'message' && json.data?.content) {
				const content = json.data.content as string;

				if (content.includes('<br>') && content.includes('opinion:')) {
					this.isParsingOpinion = true;
					this.sawOpinionMarker = true;
					this.currentOpinionContent = '';
					this.streamingClaimId = `claim-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
					this.streamingClaim = {
						id: this.streamingClaimId,
						content: ''
					};
					this.onClaimUpdate?.(this.streamingClaim, this.extractedClaims);
					return;
				}

				if (this.isParsingOpinion) {
					this.currentOpinionContent += content;
					const trimmedContent = stripOpinionPrefix(this.currentOpinionContent);
					const endsWithPunctuation = /[.!?]$/.test(trimmedContent);

					if (endsWithPunctuation && trimmedContent.length > 10) {
						this.extractedClaims = [
							...this.extractedClaims,
							{
								id: this.streamingClaimId,
								content: trimmedContent
							}
						];
						this.isParsingOpinion = false;
						this.currentOpinionContent = '';
						this.streamingClaim = null;
						this.streamingClaimId = '';
						this.onClaimUpdate?.(null, this.extractedClaims);
					} else {
						this.streamingClaim = {
							id: this.streamingClaimId,
							content: trimmedContent
						};
						this.onClaimUpdate?.(this.streamingClaim, this.extractedClaims);
					}
				} else {
					this.currentAnswer += content;
				}
			}
		} catch {}
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
		this.sawOpinionMarker = false;
		this.isParsingOpinion = false;
		this.currentOpinionContent = '';
		this.abortController = new AbortController();
	}

	async send(question: string): Promise<string> {
		if (!browser) return '';

		this.resetStreamState();

		try {
			const response = await fetch(
				`${this.baseUrl}/tools/elicitation_bot/workflow_step/${this.workflowStepId}`,
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

	/**
	 * Finalize any streaming claim that hasn't been finalized yet.
	 * Call this after send() completes since the opinion section doesn't have a message_end event.
	 */
	finalizeStreamingClaim() {
		if (this.isParsingOpinion && this.currentOpinionContent.trim()) {
			this.extractedClaims = [
				...this.extractedClaims,
				{
					id: this.streamingClaimId,
					content: this.currentOpinionContent.trim()
				}
			];
		}
		this.isParsingOpinion = false;
		this.currentOpinionContent = '';
		this.streamingClaim = null;
		this.streamingClaimId = '';
		this.onClaimUpdate?.(null, this.extractedClaims);
	}

	reset() {
		this.abort();
		this.currentAnswer = '';
		this.error = null;
		this.session = null;
	}
}
