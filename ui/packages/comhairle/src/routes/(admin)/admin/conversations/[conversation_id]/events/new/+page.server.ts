import { error, redirect } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { canPerformAction } from '$lib/utils/permissions';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, parent, locals }) => {
	await parent();
	const conversationId = params.conversation_id;
	const actions = await tryCatchAsync(() =>
		locals.api.GetUserActions({
			params: { resource_type: 'conversation', resource_id: conversationId }
		})
	);
	if (
		actions.err !== null ||
		actions.ok.resourceType !== 'conversation' ||
		actions.ok.resourceId !== conversationId
	) {
		error(HttpStatus.InternalServerError, 'Unable to load conversation permissions.');
	}

	if (!canPerformAction(actions.ok, 'conversation_update')) {
		redirect(HttpStatus.SeeOther, `/admin/conversations/${conversationId}/events`);
	}

	return {};
};
