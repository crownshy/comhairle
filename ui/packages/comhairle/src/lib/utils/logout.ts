import { goto } from '$app/navigation';
import { resolve } from '$app/paths';
import { apiClient } from '@crownshy/api-client/client';
import { m } from '$lib/paraglide/messages';
import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export async function logOut() {
	const result = await tryCatchAsync(async () => {
		await apiClient.LogoutUser(undefined);
		await goto(resolve('/'), { invalidate: [key('user')] });
	});
	if (result.err) {
		notifications.send({ priority: 'ERROR', message: m.logout_failed() });
	}
}
