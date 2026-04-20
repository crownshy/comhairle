import { toast, type ExternalToast } from 'svelte-sonner';
export { Toaster as NotificationsToaster } from '$lib/components/ui/sonner';
import { z } from 'zod';

// Use to serialize and deserialize the flash notifications
let FlashString: z.ZodType<Array<SendOpts>> = z.array(
	z.object({
		message: z.string(),
		priority: z.optional(z.enum(['INFO', 'WARNING', 'ERROR', 'SUCCESS'])),
		duration: z.optional(z.number())
	})
);

class NotificationsManager {
	private static DEFAULT_DURATION = 60000;

	public send(opts: SendOpts) {
		this.toastFn(opts.priority)(opts.message, {
			duration: opts.duration || NotificationsManager.DEFAULT_DURATION,
			action: opts.action
		});
	}

	public async listen() {
		// Notification listening is now handled by the NotificationService
		// which uses WebSockets. This method is kept for backwards compatibility
		// but doesn't need to do anything as the service auto-initializes.
		console.log('Notification listening is handled by WebSocket service');
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

	// TODO this doesn't work when running on server side
	// need to perhaps move to cookies to store and
	// retrive the flash notifications
	public addFlash(opts: SendOpts) {
		try {
			let flashString = sessionStorage.getItem('comhairle_flash_notfifications');
			let flash;
			if (flashString) {
				flash = FlashString.parse(JSON.parse(flashString));
			} else {
				flash = [opts];
			}
			sessionStorage.setItem('comhairle_flash_notfifications', JSON.stringify(flash));
		} catch (e) {
			console.warn('Failed to set session storage, probably on server');
		}
	}

	public showFlash() {
		if (!sessionStorage) return;
		let flashString = sessionStorage.getItem('comhairle_flash_notfifications');
		if (flashString) {
			let flash = FlashString.parse(JSON.parse(flashString));
			for (let message of flash) {
				this.send(message);
			}
		}
		sessionStorage.removeItem('comhairle_flash_notfifications');
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
