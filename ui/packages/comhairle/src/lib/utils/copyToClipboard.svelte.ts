import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { Second } from './units';

const COPIED_FOR = 2 * Second;

/**
 * Copies text and shows the copied state on the button itself for a moment, since info
 * toasts are hidden in this app. A failed copy says so with an error toast.
 */
export function copyToClipboard() {
	let copied = $state(false);

	async function copy(text: string) {
		const result = await tryCatchAsync(() => navigator.clipboard.writeText(text));
		if (result.err !== null) {
			notifications.send({ message: 'Could not copy the link', priority: 'ERROR' });
			return;
		}
		copied = true;
		setTimeout(() => (copied = false), COPIED_FOR);
	}

	return {
		get copied() {
			return copied;
		},
		copy
	};
}
