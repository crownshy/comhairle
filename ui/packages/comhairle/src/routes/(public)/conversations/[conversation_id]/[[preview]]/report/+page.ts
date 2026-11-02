import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { FullReportDto } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('public/report'));
	const { api, conversation } = await parent();

	const report = await tryCatchAsync(
		() =>
			api.GetReportForConversation({
				params: { conversation_id: conversation.id }
			}) as Promise<FullReportDto>
	);
	if (report.err !== null) {
		notifications.addFlash({ message: 'No such report', priority: 'WARNING' });
		redirect(302, '/');
	}

	return { conversation, report: report.ok };
};
