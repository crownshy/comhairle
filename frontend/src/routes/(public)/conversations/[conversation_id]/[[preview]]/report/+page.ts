import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent }) => {
	const { api, conversation, workflows } = await parent();
	const conversation_id = conversation.id;

	try {
		const report = await api.GetReportForConversation({
			params: { conversation_id: conversation.id }
		});
		try {
			const workflow_steps = await api.ListWorkflowSteps({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
			const workflow_stats = await api.GetWorkflowStats({
				params: { conversation_id, workflow_id: workflows[0].id }
			});

			return { conversation, workflows, workflow_steps, workflow_stats, report };
		} catch (e) {
			notifications.addFlash({ message: 'Something went wrong', priority: 'ERROR' });
		}
	} catch (e) {
		notifications.addFlash({ message: 'No such report', priority: 'WARNING' });
		redirect(302, '/');
	}
};
