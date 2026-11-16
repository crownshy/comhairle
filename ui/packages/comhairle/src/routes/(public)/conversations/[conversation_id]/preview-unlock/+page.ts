import { backToFromUrl } from '$lib/utils/backTo';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params, url }) => {
	const fallback = `/conversations/${params.conversation_id}/preview`;
	return { conversationId: params.conversation_id, backTo: backToFromUrl(url, fallback) };
};
