import { hasInsights } from '$lib/reports/insightsTools';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, workflowSteps } = await parent();
	const insightsSteps = workflowSteps.flatMap((step) => {
		const toolType = (conversation.isLive ? step.toolConfig : step.previewToolConfig)?.type;
		return hasInsights(toolType) ? [{ id: step.id, name: step.name, toolType }] : [];
	});
	return { insightsSteps };
};
