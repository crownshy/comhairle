import type { PageLoad } from './$types';

export const load: PageLoad = ({ params, url }) => {
	const fallback = `/conversations/${params.conversation_id}/preview`;
	const requested = url.searchParams.get('backTo') ?? fallback;
	// Only same-site paths, so the form can't be used to bounce people off to another site.
	const backTo = requested.startsWith('/') && !requested.startsWith('//') ? requested : fallback;
	return { conversationId: params.conversation_id, backTo };
};
