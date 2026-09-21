import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { LoadEvent } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import type { ChatInstructionsDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, depends, params }: LoadEvent) => {
	depends(key('knowledge-base/documents'));

	const { conversation_id } = params;
	if (!conversation_id) {
		return;
	}

	const { api } = await parent();

	const docsResponse = await tryCatchAsync(() =>
		api.ListDocuments({
			params: { conversation_id }
		})
	);

	if (docsResponse.err !== null) {
		console.error(docsResponse.err);
		return;
	}

	const chatResponse = await tryCatchAsync(() =>
		api.GetChat({
			params: { conversation_id }
		})
	);

	if (chatResponse.err !== null) {
		console.error(chatResponse.err);
		return;
	}

	let chatInstructions: ChatInstructionsDto | null = null;
	const chatInstructionsResponse = await tryCatchAsync(() =>
		api.GetConversationChatInstructions({ params: { conversation_id } })
	);

	if (chatInstructionsResponse.err !== null) {
		console.error(chatResponse.err);
	} else {
		chatInstructions = chatInstructionsResponse.ok as ChatInstructionsDto;
	}

	return {
		documents: docsResponse.ok,
		chat: chatResponse.ok,
		chatInstructions: chatInstructions
	};
};
