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

export class WSConnection {
	socket: WebSocket | null = null;
	connectionStatus = $state<'disconnected' | 'connecting' | 'connected' | 'error'>(
		'disconnected'
	);
	reconnectAttempts = $state(0);

	private messageHandlers: Set<MessageHandler> = new Set();
	private typedHandlers: Map<string, Set<TypedMessageHandler<any>>> = new Map();
	private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000;
	private pingInterval: ReturnType<typeof setInterval> | null = null;

	connect() {
		if (!browser) {
			console.log('WebSocket only available in browser');
			return;
		}

		if (this.socket?.readyState === WebSocket.OPEN || this.connectionStatus === 'connecting') {
			return;
		}

		this.connectionStatus = 'connecting';

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
			this.stopPingInterval();
			this.attemptReconnect();
		};
	}

	disconnect() {
		if (this.reconnectTimeout) {
			clearTimeout(this.reconnectTimeout);
			this.reconnectTimeout = null;
		}
		this.stopPingInterval();
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
		this.connectionStatus = 'disconnected';
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

	// Convenience methods for common message types
	sendCustom(event: string, data: any) {
		this.send({
			type: 'custom',
			payload: { event, data }
		});
	}
}

// Singleton instance - available everywhere
export const ws = new WSConnection();

// Auto-connect in browser
if (browser) {
	ws.connect();
}
