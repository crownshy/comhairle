import { browser } from '$app/environment';

// Core types for base WebSocket service
export type NotificationLevel = 'info' | 'warning' | 'error' | 'success';

// Core message types - only protocol-level messages
export type CoreWebSocketMessage =
	| { type: 'ping'; payload: { timestamp: number } }
	| { type: 'pong'; payload: { timestamp: number } }
	| {
			type: 'notification';
			payload: { title: string; message: string; level: NotificationLevel };
	  }
	| { type: 'user_joined'; payload: { user_id: string; username?: string } }
	| { type: 'user_left'; payload: { user_id: string; username?: string } }
	| { type: 'broadcast'; payload: { message: string; from_user?: string } }
	| { type: 'error'; payload: { code: string; message: string } }
	| { type: 'custom'; payload: { event: string; data: any } };

// Allow extension by other services
export type WebSocketMessage = CoreWebSocketMessage | { type: string; payload: any };

type MessageHandler = (message: WebSocketMessage) => void;
type TypedMessageHandler<T extends WebSocketMessage['type']> = (
	payload: Extract<WebSocketMessage, { type: T }>['payload']
) => void;
type CloseHandler = (event: CloseEvent) => void;

type PendingCustomRequest = {
	resolve: (value: any) => void;
	reject: (reason?: unknown) => void;
	timeoutId: ReturnType<typeof setTimeout>;
	expectedEvent: string;
};

type CustomResultEnvelope<T> = {
	requestId: string;
	success: boolean;
	data: T | null;
	error: string | null;
};

export class WSConnection {
	socket: WebSocket | null = null;
	connectionStatus = $state<'disconnected' | 'connecting' | 'connected' | 'error'>(
		'disconnected'
	);
	reconnectAttempts = $state(0);

	private messageHandlers: Set<MessageHandler> = new Set();
	private typedHandlers: Map<string, Set<TypedMessageHandler<any>>> = new Map();
	private closeHandlers: Set<CloseHandler> = new Set();
	private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000;
	private pingInterval: ReturnType<typeof setInterval> | null = null;
	private intentionalDisconnect = false;
	private pendingCustomRequests: Map<string, PendingCustomRequest> = new Map();

	connect() {
		if (!browser) {
			console.log('WebSocket only available in browser');
			return;
		}

		if (this.socket?.readyState === WebSocket.OPEN || this.connectionStatus === 'connecting') {
			return;
		}

		this.connectionStatus = 'connecting';
		this.intentionalDisconnect = false;

		// In development, bypass Vite proxy and connect directly to backend
		// Cookies are sent automatically with WebSocket connections
		const isDev = import.meta.env.DEV;
		const url = isDev
			? 'ws://localhost:3000/ws'
			: `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/api/ws`;

		console.log(
			'Connecting to WebSocket:',
			url,
			isDev ? '(dev - direct)' : '(prod - via proxy)'
		);

		this.socket = new WebSocket(url);

		this.socket.onopen = () => {
			console.log('WebSocket connection opened');
			this.connectionStatus = 'connected';
			this.reconnectAttempts = 0;
			this.startPingInterval();
		};

		this.socket.onmessage = (event) => {
			try {
				const message: WebSocketMessage = JSON.parse(event.data);
				console.log('WebSocket message received:', message);

				// Call general message handlers
				this.messageHandlers.forEach((handler) => handler(message));

				// Call typed handlers for this message type
				const handlers = this.typedHandlers.get(message.type);
				if (handlers) {
					handlers.forEach((handler) => handler(message.payload));
				}

				if (message.type === 'custom') {
					this.handleCustomResultMessage(message.payload.event, message.payload.data);
				}

				// Handle pong responses for ping
				if (message.type === 'pong') {
					console.log('Received pong');
				}
			} catch (error) {
				console.error('Error parsing WebSocket message:', error);
			}
		};

		this.socket.onerror = (error) => {
			console.error('WebSocket error:', error);
			this.connectionStatus = 'error';
		};

		this.socket.onclose = (event) => {
			console.log('WebSocket connection closed:', event.code, event.reason);
			this.connectionStatus = 'disconnected';
			this.closeHandlers.forEach((handler) => handler(event));
			this.rejectPendingCustomRequests('WebSocket connection closed');
			this.stopPingInterval();
			if (this.intentionalDisconnect) {
				this.intentionalDisconnect = false;
				return;
			}
			this.attemptReconnect();
		};
	}

	disconnect() {
		this.intentionalDisconnect = true;
		if (this.reconnectTimeout) {
			clearTimeout(this.reconnectTimeout);
			this.reconnectTimeout = null;
		}
		this.stopPingInterval();
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
		this.rejectPendingCustomRequests('WebSocket disconnected');
		this.connectionStatus = 'disconnected';
	}

	private handleCustomResultMessage(event: string, data: any) {
		if (!event.endsWith('_result')) return;
		if (!data || typeof data !== 'object') return;

		const requestId = data.requestId;
		if (typeof requestId !== 'string' || requestId.length === 0) return;

		const pending = this.pendingCustomRequests.get(requestId);
		if (!pending) return;
		if (pending.expectedEvent !== event) return;

		this.pendingCustomRequests.delete(requestId);
		clearTimeout(pending.timeoutId);

		const envelope = data as CustomResultEnvelope<any>;
		if (!envelope.success) {
			pending.reject(new Error(envelope.error ?? `Request failed for ${event}`));
			return;
		}

		pending.resolve(envelope.data);
	}

	private rejectPendingCustomRequests(reason: string) {
		for (const [, pending] of this.pendingCustomRequests) {
			clearTimeout(pending.timeoutId);
			pending.reject(new Error(reason));
		}
		this.pendingCustomRequests.clear();
	}

	private async waitUntilConnected(timeoutMs: number): Promise<void> {
		if (this.socket?.readyState === WebSocket.OPEN) return;
		this.connect();

		const startedAt = Date.now();
		while (Date.now() - startedAt < timeoutMs) {
			if (this.socket?.readyState === WebSocket.OPEN) return;
			await new Promise<void>((resolve) => setTimeout(resolve, 25));
		}

		throw new Error('WebSocket connection timed out');
	}

	private attemptReconnect() {
		if (this.reconnectAttempts >= this.maxReconnectAttempts) {
			console.error('Max reconnection attempts reached');
			return;
		}

		this.reconnectAttempts++;
		const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
		console.log(`Attempting to reconnect in ${delay}ms (attempt ${this.reconnectAttempts})`);

		this.reconnectTimeout = setTimeout(() => {
			this.connect();
		}, delay);
	}

	private startPingInterval() {
		this.pingInterval = setInterval(() => {
			this.send({ type: 'ping', payload: { timestamp: Date.now() } });
		}, 30000); // Ping every 30 seconds
	}

	private stopPingInterval() {
		if (this.pingInterval) {
			clearInterval(this.pingInterval);
			this.pingInterval = null;
		}
	}

	send(message: WebSocketMessage) {
		if (this.socket && this.socket.readyState === WebSocket.OPEN) {
			this.socket.send(JSON.stringify(message));
		} else {
			console.warn('WebSocket not open; cannot send message');
		}
	}

	// Subscribe to all messages
	onMessage(handler: MessageHandler): () => void {
		this.messageHandlers.add(handler);
		return () => this.messageHandlers.delete(handler);
	}

	// Subscribe to specific message types
	on<T extends WebSocketMessage['type']>(type: T, handler: TypedMessageHandler<T>): () => void {
		if (!this.typedHandlers.has(type)) {
			this.typedHandlers.set(type, new Set());
		}
		this.typedHandlers.get(type)!.add(handler);

		return () => {
			const handlers = this.typedHandlers.get(type);
			if (handlers) {
				handlers.delete(handler);
				if (handlers.size === 0) {
					this.typedHandlers.delete(type);
				}
			}
		};
	}

	// Subscribe to socket close events
	onClose(handler: CloseHandler): () => void {
		this.closeHandlers.add(handler);
		return () => this.closeHandlers.delete(handler);
	}

	// Convenience methods for common message types
	sendCustom(event: string, data: any) {
		this.send({
			type: 'custom',
			payload: { event, data }
		});
	}

	async requestCustom<TResponse>(
		event: string,
		data: Record<string, unknown>,
		options?: { timeoutMs?: number; responseEvent?: string }
	): Promise<TResponse> {
		const timeoutMs = options?.timeoutMs ?? 15_000;
		const responseEvent = options?.responseEvent ?? `${event}_result`;
		const requestId =
			browser && typeof crypto.randomUUID === 'function'
				? crypto.randomUUID()
				: `${Date.now()}-${Math.random().toString(16).slice(2)}`;

		await this.waitUntilConnected(timeoutMs);

		return new Promise<TResponse>((resolve, reject) => {
			const timeoutId = setTimeout(() => {
				this.pendingCustomRequests.delete(requestId);
				reject(new Error(`Timed out waiting for ${responseEvent}`));
			}, timeoutMs);

			this.pendingCustomRequests.set(requestId, {
				resolve,
				reject,
				timeoutId,
				expectedEvent: responseEvent
			});

			this.sendCustom(event, {
				requestId,
				...data
			});
		});
	}
}

// Singleton instance - available everywhere
export const ws = new WSConnection();

// Auto-connect in browser
if (browser) {
	ws.connect();
}
