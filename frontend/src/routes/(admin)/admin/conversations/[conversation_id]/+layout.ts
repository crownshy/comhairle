import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type {
	ConversationWithTranslations,
	WorkflowDto,
	WorkflowStats,
	WorkflowStepWithTranslations
} from '$lib/api/api';

export const load: PageLoad = async ({
	params,
	parent
}): Promise<{
	conversation: ConversationWithTranslations;
	workflows: WorkflowDto[];
	workflowSteps: WorkflowStepWithTranslations[];
	stats: WorkflowStats;
}> => {
	const conversation_id = params.conversation_id;
	const { api } = await parent();

	try {
		const conversation = (await api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})) as ConversationWithTranslations;
		const workflows = await api.ListWorkflows({ params: { conversation_id } });
		let stats = undefined;
		let workflowSteps = undefined;

		if (workflows.length > 0) {
			stats = await api.GetWorkflowStats({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
			workflowSteps = await api.ListWorkflowSteps({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
		}
		return { conversation, workflows, stats, workflowSteps };
	} catch (e) {
		console.error(e);
		notifications.addFlash({
			message: 'Problem loading conversation assets',
			priority: 'WARNING'
		});
		redirect(302, '/admin');
	}
};
