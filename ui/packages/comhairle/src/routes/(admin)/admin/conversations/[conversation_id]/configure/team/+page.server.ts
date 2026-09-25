import { resolve } from '$app/paths';
import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync, tryFetch } from '$lib/utils/errorHandling';
import type { ConversationDto } from '@crownshy/api-client/api';
import { redirect, type ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = async ({ parent, params, fetch }) => {
	const { user } = await parent();
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

	const conversationResponse = await tryFetch(
		`/api/conversation/${conversation_id}?withTranslations=true`,
		undefined,
		fetch
	);

	if (conversationResponse.err !== null) {
		redirect(HttpStatus.Found, configurePage);
	}

	const conversation = await tryCatchAsync(() => conversationResponse.ok.json());

	if (conversation.err !== null || user.id !== (conversation.ok as ConversationDto).ownerId) {
		redirect(HttpStatus.Found, configurePage);
	}
};
