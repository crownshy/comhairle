import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';
import type { MediaDto } from '@crownshy/api-client/api';

/**
 * The Content tab's rich fields (FAQ, thank-you, privacy policy, short privacy policy) offer an
 * "Insert Source Document" control. It needs the conversation's parsed knowledge base documents to
 * populate the picker, and it needs them again on the render path to resolve each badge's name/size
 * and download link. We fetch them once here (page-scoped so unrelated conversation sub-pages don't
 * pay for it) and only surface the DONE-parsed ones, matching the Learn step path. A failed fetch
 * falls back to an empty list, so the picker shows its empty state rather than a raw backend error.
 */
export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('conversation/documents'));

	const { api, conversation } = await parent();
	const { conversation_id } = params;

	const documents = await tryCatchAsync(() =>
		api.ListDocuments({
			params: { conversation_id }
		})
	);

	if (documents.err !== null) {
		console.warn('failed to load conversation documents', documents.err);
	}

	const availableDocuments = documents.ok?.filter((d) => d.parse_status === 'DONE') ?? [];

	let media: MediaDto | null = null;
	const { image } = conversation;
	if (image) {
		const result = await tryCatchAsync(() => api.GetMedia({ params: { media_id: image } }));
		media = result.ok;
	}

	return {
		availableDocuments,
		media
	};
};
