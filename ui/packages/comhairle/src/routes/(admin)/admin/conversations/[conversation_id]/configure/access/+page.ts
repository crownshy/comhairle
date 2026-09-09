import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';
import { notifications } from '$lib/notifications.svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { resolve } from '$app/paths';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();

	const cohostOrganizations = await tryCatchAsync(() =>
		api.ListConversationCoHostOrganizations({
			params: { conversation_id: params.conversation_id }
		})
	);

	if (cohostOrganizations.err !== null) {
		console.error(cohostOrganizations.err);
		notifications.addFlash({
			message: 'Problem loading cohost organisations',
			priority: 'WARNING'
		});
		redirect(
			HttpStatus.Found,
			resolve('/(admin)/admin/conversations/[conversation_id]/configure/details', {
				conversation_id: params.conversation_id
			})
		);
	}
};
