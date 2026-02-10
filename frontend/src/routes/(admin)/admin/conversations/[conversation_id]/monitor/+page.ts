import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { conversation, workflows, workflow_steps, api } = await parent();
	const workflowStats = await api.GetWorkflowStats({
		params: { conversation_id: conversation.id, workflow_id: workflows[0].id }
	});
	try {
		const conversationStats = await api.GetWorkflowStats({
			params: { conversation_id: conversation.id, workflow_id: workflows[0].id }
		});

		return { conversation, workflows, workflow_steps, workflowStats, conversationStats };
	} catch (e) {
		console.error(e);
	}
};
