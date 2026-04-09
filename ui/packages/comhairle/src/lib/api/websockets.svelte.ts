import { browser } from '$app/environment';

export type WSMessageType =
	| 'ping'
	| 'pong'
	| 'notification'
	| 'user_started_workflow_step'
	| 'user_finished_workflow_step'
	| 'user_idle'
	| 'user_joined'
	| 'user_left'
	| 'broadcast'
	| 'error'
	| 'custom';

export interface WSMessage {
	type: WSMessageType;
	payload: Record<string, any>;
}

type MessageHandler = (message: WSMessage) => void;

export class WSConnection {
	socket: WebSocket | null = null;
	connectionStatus: 'pending' | 'ready' | 'closed' = $state('pending');
	messages: WSMessage[] = $state([]);

	private handlers: MessageHandler[] = [];
	private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 10;
	private static readonly MAX_MESSAGES = 200;
	private static readonly MAX_SEND_RETRIES = 5;

	connect() {
		if (!browser) return;
		this.maxReconnectAttempts = 10;

		const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
		const host = window.location.host;
		const token = document.cookie
			.split('; ')
			.find((row) => row.startsWith('access_token='))
			?.split('=')[1];
		const url = `${protocol}//${host}/api/ws${token ? `?token=${token}` : ''}`;
		this.socket = new WebSocket(url);

		this.socket.onopen = () => {
			console.log('WS connection opened');
			this.connectionStatus = 'ready';
			this.reconnectAttempts = 0;
		};

		this.socket.onmessage = (event) => {
			try {
				const message: WSMessage = JSON.parse(event.data);

				// Auto-respond to pings
				if (message.type === 'ping') {
					this.send({ type: 'pong', payload: message.payload });
					return;
				}

				this.messages = [...this.messages.slice(-WSConnection.MAX_MESSAGES + 1), message];
				this.handlers.forEach((handler) => handler(message));
			} catch (e) {
				console.warn('WS: failed to parse message', event.data, e);
			}
		};

		this.socket.onclose = () => {
			console.log('WS connection closed');
			this.connectionStatus = 'closed';
			this.scheduleReconnect();
		};

		this.socket.onerror = (e) => {
			console.log('WS error', e);
		};
	}

	private scheduleReconnect() {
		if (this.reconnectAttempts >= this.maxReconnectAttempts) return;
		const delay = Math.min(1000 * 2 ** this.reconnectAttempts, 30000);
		this.reconnectAttempts++;
		this.reconnectTimer = setTimeout(() => this.connect(), delay);
	}

	onMessage(handler: MessageHandler) {
		this.handlers.push(handler);
		return () => {
			this.handlers = this.handlers.filter((h) => h !== handler);
		};
	}

	send(data: any, _retries = 0) {
		if (this.socket && this.socket.readyState === WebSocket.OPEN) {
			this.socket.send(JSON.stringify(data));
		} else if (_retries < WSConnection.MAX_SEND_RETRIES) {
			console.warn(
				`WebSocket not open; retry ${_retries + 1}/${WSConnection.MAX_SEND_RETRIES}...`
			);
			setTimeout(() => this.send(data, _retries + 1), 500);
		} else {
			console.error('WebSocket not open after max retries; message dropped.');
		}
	}

	disconnect() {
		if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
		this.maxReconnectAttempts = 0;
		this.socket?.close();
	}
}

export const ws = new WSConnection();
ws.connect();
