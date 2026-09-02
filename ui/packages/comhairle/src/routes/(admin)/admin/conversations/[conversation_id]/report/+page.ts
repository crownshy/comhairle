import { FullReportDto } from '@crownshy/api-client/api';
import type { PageLoad } from './$types';
import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
import { key } from '$lib/utils/invalidationKey';

// Tools that have embeddable report components today. Grows as more tools get a
// component set (Thinking Space is next); HeyForm has none yet.
const REPORT_CAPABLE_TOOLS = new Set(['polis']);

export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('conversation/report'));
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
