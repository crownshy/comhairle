import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { ConversationWithTranslations, Workflow, WorkflowStep } from '$lib/api/api';

export const load: PageLoad = async ({
	params,
	parent
}): Promise<{
	conversation: ConversationWithTranslations;
	workflows: Workflow[];
	workflow_steps: WorkflowStep[];
	stats: any; // TODO:
}> => {
	const conversation_id = params.conversation_id;
	const { api } = await parent();
	try {
		const conversation = await api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		});
		const workflows = await api.ListWorkflows({ params: { conversation_id } });
		let stats = undefined;
		let workflow_steps = undefined;

		if (workflows.length > 0) {
			stats = await api.GetWorkflowStats({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
			workflow_steps = await api.ListWorkflowSteps({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
		}
		return { conversation, workflows, stats, workflow_steps };
	} catch (e) {
		console.error(e);
		notifications.addFlash({
			message: 'Problem loading conversation assets',
			priority: 'WARNING'
		});
		redirect(302, '/admin');
	}
};
