import { ws } from '$lib/api/websockets.svelte';
import { notifications, type NotificationPriorities } from '$lib/notifications.svelte';

interface ServerNotification {
	id: string;
	title: string;
	message: string;
	level: string;
}

export class NotificationService {
	private _unreadCount = $state(0);
	private isListening = false;

	constructor() {
		this.setupListeners();
		// Request initial count after a short delay to ensure WebSocket is connected
		setTimeout(() => this.requestUnreadCount(), 100);
	}

	get unreadCount() {
		return this._unreadCount;
	}

	private setupListeners() {
		if (this.isListening) return;
		this.isListening = true;

		// Listen for new notifications from server
		ws.on('custom', (payload) => {
			if (payload.event === 'notification:new') {
				this.handleNewNotification(payload.data as ServerNotification);
			} else if (payload.event === 'notification:unread_count') {
				this._unreadCount = payload.data.count;
			} else if (payload.event === 'notification:marked_read') {
				console.log('Notification marked as read:', payload.data);
			} else if (payload.event === 'notification:all_marked_read') {
				console.log('All notifications marked as read:', payload.data);
				this._unreadCount = 0;
			}
		});

		// Listen for regular notification messages
		ws.on('notification', (payload) => {
			this.handleNotificationMessage(payload);
		});

		// Request initial unread count when connected
		this.requestUnreadCount();
	}

	private handleNewNotification(notification: ServerNotification) {
		// Map backend level to frontend priority
		const priority = this.mapLevelToPriority(notification.level);

		// Show toast notification
		notifications.send({
			message: `${notification.title}: ${notification.message}`,
			priority
		});

		// Increment unread count
		this._unreadCount++;
	}

	private handleNotificationMessage(payload: { title: string; message: string; level: string }) {
		const priority = this.mapLevelToPriority(payload.level);
		notifications.send({
			message: `${payload.title}: ${payload.message}`,
			priority
		});
	}

	private mapLevelToPriority(level: string): NotificationPriorities {
		switch (level.toLowerCase()) {
			case 'success':
				return 'SUCCESS';
			case 'warning':
				return 'WARNING';
			case 'error':
				return 'ERROR';
			case 'info':
			default:
				return 'INFO';
		}
	}

	// Mark a notification as read
	markAsRead(deliveryId: string) {
		ws.sendCustom('notification:mark_read', {
			delivery_id: deliveryId
		});
	}

	// Mark all notifications as read
	markAllAsRead() {
		ws.sendCustom('notification:mark_all_read', {});
	}

	// Request current unread count
	requestUnreadCount() {
		if (ws.connectionStatus === 'connected') {
			ws.sendCustom('notification:get_unread_count', {});
		} else {
			// Retry when connected
			setTimeout(() => this.requestUnreadCount(), 1000);
		}
	}
}

// Singleton instance
export const notificationService = new NotificationService();
