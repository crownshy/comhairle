import type { PageLoad } from './$types';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
<<<<<<< HEAD
import type { ConversationWithTranslations, Workflow, WorkflowStep } from '$lib/api/api';
||||||| parent of a147df8 (221 update ListWorkflow route and frontend usage to use dto with camelCase fields)
=======
import type { WorkflowDto } from '$lib/api/api';
>>>>>>> a147df8 (221 update ListWorkflow route and frontend usage to use dto with camelCase fields)

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
<<<<<<< HEAD
		const workflows = await api.ListWorkflows({ params: { conversation_id } });
||||||| parent of a147df8 (221 update ListWorkflow route and frontend usage to use dto with camelCase fields)
		let workflows = await api.ListWorkflows({ params: { conversation_id } });
=======
		let workflows: WorkflowDto[] = await api.ListWorkflows({ params: { conversation_id } });
>>>>>>> a147df8 (221 update ListWorkflow route and frontend usage to use dto with camelCase fields)
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
