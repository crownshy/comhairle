import { canPerformAction } from '$lib/utils/permissions';
import { error } from '@sveltejs/kit';
import { FullReportDto } from '@crownshy/api-client/api';
import { HttpStatus } from '$lib/utils/constants';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { hasReportWidgets } from '$lib/reports/embeds';
import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
import type { PageLoad } from './$types';

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
		.filter((step) => hasReportWidgets(step.toolConfig?.type))
		.map((step) => ({
			id: step.id,
			conversationId: conversation.id,
			workflowId: step.workflowId,
			name: step.name,
			toolType: step.toolConfig!.type,
			reportDataPublic: step.reportDataPublic
		}));

	return { report, conversation, reportEmbedSteps };
};
