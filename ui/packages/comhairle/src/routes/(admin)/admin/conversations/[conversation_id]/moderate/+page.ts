import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, workflowSteps } = await parent();
	const polisSteps = workflowSteps.filter((step) => {
		const toolConfig = conversation.isLive ? step.toolConfig : step.previewToolConfig;
		return toolConfig?.type === 'polis';
	});
	return { polisSteps };
};
