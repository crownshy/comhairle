import { hasSectionBlocks } from '$lib/reports/embeds';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, depends, params }) => {
	depends(key('admin/conversation/report'));
	const { api, toolConfig } = await parent();
	if (!hasSectionBlocks(toolConfig?.type)) return { reportEmbeds: null };

	const result = await tryCatchAsync(() =>
		api.GetReportEmbeds({ params: { conversation_id: params.conversation_id } })
	);
	return { reportEmbeds: result.err === null ? result.ok : null };
};
