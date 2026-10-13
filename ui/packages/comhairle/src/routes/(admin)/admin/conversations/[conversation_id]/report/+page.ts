import { canPerformAction } from '$lib/utils/permissions';
import { error } from '@sveltejs/kit';
import { FullReportDto } from '@crownshy/api-client/api';
import { HttpStatus } from '$lib/utils/constants';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
import type { PageLoad } from './$types';

// Tools that have embeddable report components today. Grows as more tools get a
// component set (Thinking Space is next); HeyForm has none yet.
const REPORT_CAPABLE_TOOLS = new Set(['polis']);

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversation/report'));
	const { conversation, api, workflowSteps, conversationActions } = await parent();
	let report: FullReportDto;

	const result = await tryCatchAsync(() =>
		api.GetReportForConversation({
			params: { conversation_id: conversation.id },
			queries: { withTranslations: true }
		})
	);
	if (result.err === null) {
		report = result.ok;
	} else {
		if (!canPerformAction(conversationActions, 'conversation_update')) {
			error(HttpStatus.Forbidden, 'User lacks permission to update conversation');
		}
		report = await api.GenerateReportForConversation(undefined, {
			params: { conversation_id: conversation.id }
		});
	}

	// Steps offered by the "Embed report component" control, resolved to the shape it needs.
	const reportEmbedSteps: EmbeddableStep[] = (workflowSteps ?? [])
		.filter((step) => {
			const toolType = step.toolConfig?.type;
			return toolType != null && REPORT_CAPABLE_TOOLS.has(toolType);
		})
		.map((step) => ({
			id: step.id,
			name: step.name,
			toolType: step.toolConfig!.type
		}));

	return { report, conversation, reportEmbedSteps };
};
