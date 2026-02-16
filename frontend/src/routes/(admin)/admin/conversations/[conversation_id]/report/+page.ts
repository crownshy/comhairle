import { FullReportDto } from '$lib/api/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, api } = await parent();
	let report: FullReportDto;

	try {
		report = await api.GetReportForConversation({
			params: { conversation_id: conversation.id }
		});
	} catch (e) {
		report = await api.GenerateReportForConversation(undefined, {
			params: { conversation_id: conversation.id }
		});
	}
	return { report, conversation };
};
