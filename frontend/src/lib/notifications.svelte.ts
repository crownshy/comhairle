import { toast } from 'svelte-sonner';
export { Toaster as NotificationsToaster } from '$lib/components/ui/sonner';

class NotificationsManager {
	private static DURATION = 60000;

	public send(msg: string) {
		toast(msg, {
			duration: NotificationsManager.DURATION
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

	private sendServerNotification(msg: string, id: string) {
		toast(msg, {
			duration: NotificationsManager.DURATION,
			onDismiss: () => {
				this.ack(id);
			},
			onAutoClose: () => {
				this.ack(id);
			}
		});
	}

	private async ack(id: string) {
		console.log(`acknowlegding server notification ${id}`);
		// TODO: implement
	}
}

// `notifications` is the central notifications manager for the project. It can
// listen for notifications from the server, and it can send notifications
// within the local app.
//
// To send a notification within the app:
//   notifications.send("my message");
//
// To listen for notifications from the server (should be done once):
//   $effect(() => { notifications.listen(); });
export const notifications = new NotificationsManager();
