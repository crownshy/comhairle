import { tryCatchAsync } from '$lib/utils/errorHandling';
import { HttpStatus } from '$lib/utils/constants';
import { key } from '$lib/utils/invalidationKey';
import { resolve } from '$app/paths';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('event/recordings'));

	const { api } = await parent();
	const { conversation_id, event_id } = params;

	const recordings = await tryCatchAsync(() =>
		api.ListAudioRecordings({
			params: { conversation_id, event_id }
		})
	);

	if (recordings.err !== null) {
		notifications.addFlash({
			message: 'Could not load recordings, please try again',
			priority: 'ERROR'
		});
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/events/[event_id]', {
				conversation_id,
				event_id
			})
		);
	}

	return {
		recordings: recordings.ok
	};
};
