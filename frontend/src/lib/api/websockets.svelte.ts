import { browser } from '$app/environment';
export class WSConnection {
	socket: (WebSocket | null) = null;
	connectionStatus: "pending" | "ready" = $state("pending")

	connect() {
		if (browser) {

			const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
			const host = window.location.host;
			const token = document.cookie.split('; ').find(row => row.startsWith('access_token='))?.split('=')[1];
			const url = `${protocol}//${host}/api/ws${token ? `?token=${token}` : ''}`;
			this.socket = new WebSocket(url);
			this.socket.onopen = () => {
				console.log("Connection opened")
			}
			this.socket.onerror = (e) => {
				console.log("WS error ", e)
			}
		}
		else {
			console.log("on server side")
		}
	}

	send(data: any) {
		if (this.socket && this.socket.readyState === WebSocket.OPEN) {
			this.socket.send(JSON.stringify(data));
		} else {
			console.warn('WebSocket not open; retrying soon...');
			setTimeout(() => this.send(data), 500);
		}
	}
}

export const ws = new WSConnection()
ws.connect()
