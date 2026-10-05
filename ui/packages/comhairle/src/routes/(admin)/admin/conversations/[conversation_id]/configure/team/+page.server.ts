import { resolve } from '$app/paths';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { canPerformAction } from '$lib/utils/permissions';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent, params, locals }) => {
	const { conversation_id } = params;

	if (!conversation_id) {
		redirect(HttpStatus.Found, resolve('/(admin)/admin/conversations'));
	}

	const configurePage = resolve(
		'/(admin)/admin/conversations/[conversation_id]/configure/details',
		{
			conversation_id
		}
	);

	await parent();

	const conversation = await tryCatchAsync(() =>
		locals.api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})
	);
	if (conversation.err !== null) {
		redirect(HttpStatus.Found, configurePage);
	}
	const actions = await tryCatchAsync(() =>
		locals.api.GetUserActions({
			params: { resource_type: 'conversation', resource_id: conversation.ok.id }
		})
	);
	if (actions.err !== null || !canPerformAction(actions.ok, 'conversation_admin')) {
		redirect(HttpStatus.Found, configurePage);
	}
};
