import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { calcOffset, getPage } from '$lib/pagination';
import QueryParams from '$lib/interfaces/QueryParams.svelte';
import type { ConversationsParams } from '$lib/interfaces/QueryParams.types';
import AdminConversation from './AdminConversation';

const PAGE_SIZE = 10;

export const load: PageLoad = async ({ parent, url, depends }) => {
	depends(key('admin/conversations'));

	const { api, user } = await parent();
	const page = getPage(url);

	return {
		conversations: await tryCatchAsync(() =>
			api.GetPermittedConversations({
				queries: {
					...AdminConversation.getParams(
						QueryParams.get<ConversationsParams>('filter', url),
						user?.id
					),
					keyword: QueryParams.get<ConversationsParams>('search', url),
					limit: PAGE_SIZE,
					offset: calcOffset({ page, pageSize: PAGE_SIZE })
				}
			})
		),
		PAGE_SIZE
	};
};
