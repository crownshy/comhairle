import type { LayoutLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import { key } from '$lib/utils/invalidationKey';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { resolve } from '$app/paths';
import type { EventWithTranslations } from '@crownshy/api-client/api';

export const load: LayoutLoad = async ({ params, parent, depends }) => {
	depends(key('conversation/event'));

	const { api } = await parent();
	const { conversation_id, event_id } = params;

	const event = await tryCatchAsync(() =>
		api.GetEvent({
			params: { conversation_id, event_id },
			queries: { withTranslations: true }
		})
	);

	if (event.err !== null) {
		notifications.addFlash({ priority: 'WARNING', message: 'Problem loading event' });
		redirect(
			HttpStatus.Found,
			resolve('/(admin)/admin/conversations/[conversation_id]/events', { conversation_id })
		);
	}

	return {
		event: event.ok as EventWithTranslations
	};
};
