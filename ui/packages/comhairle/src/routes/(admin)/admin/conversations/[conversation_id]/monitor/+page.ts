import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, workflows, workflowSteps, api } = await parent();

	try {
		const workflowStats = await api.GetConversationWorkflowStats({
			params: { conversation_id: conversation.id, workflow_id: workflows[0].id }
		});

		return { conversation, workflows, workflowSteps, workflowStats };
	} catch (e) {
		console.error(e);
	}
};
