import { FullReportDto } from '@crownshy/api-client/api';
import type { PageLoad } from './$types';
import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
import { key } from '$lib/utils/invalidationKey';
import { hasReportWidgets } from '$lib/reports/embeds';

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/conversation/report'));
	const { conversation, api, workflowSteps } = await parent();
	let report: FullReportDto;

	try {
		report = await api.GetReportForConversation({
			params: { conversation_id: conversation.id },
			queries: { withTranslations: true }
		});
	} catch {
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
