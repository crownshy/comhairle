import { toast, type ExternalToast } from 'svelte-sonner';
export { Toaster as NotificationsToaster } from '$lib/components/ui/sonner';

class NotificationsManager {
	private static DEFAULT_DURATION = 60000;

	public send(opts: SendOpts) {
		this.toastFn(opts.priority)(opts.message, {
			duration: opts.duration || NotificationsManager.DEFAULT_DURATION,
			action: opts.action
		});
	}

	public async listen() {
		console.log('listening for server notifications (not implemented');
		// TODO: implement

		// // hacky testing snippet:
		// let id = 0;
		// setInterval(() => {
		// 	++id;
		// 	this.sendServerNotification(`server message ${id}`, `${id}`);
		// }, 6000);
	}

	private sendServerNotification(opts: SendServerNotificationOpts) {
		this.toastFn(opts.priority)(opts.message, {
			duration: opts.duration || NotificationsManager.DEFAULT_DURATION,
			onDismiss: () => {
				this.ack(opts.id);
			},
			onAutoClose: () => {
				this.ack(opts.id);
			}
		});
	}

	private async ack(id: string) {
		console.log(`acknowlegding server notification ${id}`);
		// TODO: implement
	}

	private toastFn(priority?: NotificationPriorities) {
		switch (priority) {
			case 'ERROR': {
				return toast.error;
			}
			case 'INFO': {
				return toast.info;
			}
			case 'SUCCESS': {
				return toast.success;
			}
			case 'WARNING': {
				return toast.warning;
			}
			default: {
				return toast;
			}
		}
	}
}

type SendOpts = BaseOpts & { action?: ExternalToast['action'] };
type SendServerNotificationOpts = BaseOpts & { id: string };

type BaseOpts = {
	message: string;
	priority?: NotificationPriorities;
	duration?: number;
};
export type NotificationPriorities = 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';

// `notifications` is the central notifications manager for the project. It can
// listen for notifications from the server, and it can send notifications
// within the local app.
//
// To send a notification within the app:
//   notifications.send({ message: "my message" });
//
// To listen for notifications from the server (should be done once):
//   $effect(() => { notifications.listen(); });
export const notifications = new NotificationsManager();
