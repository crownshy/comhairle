import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { FullReportDto } from '@crown-shy/api-client/api';
import type { LocalizedWorkflowStepDto } from '@crown-shy/api-client/api';

export const load: PageLoad = async ({ params, parent }) => {
	const { api, conversation, workflows } = await parent();
	const conversation_id = conversation.id;

	try {
		const report: FullReportDto = await api.GetReportForConversation({
			params: { conversation_id: conversation.id }
		});
		try {
			const workflowSteps: LocalizedWorkflowStepDto[] = await api.ListWorkflowSteps({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
			const workflowStats = await api.GetWorkflowStats({
				params: { conversation_id, workflow_id: workflows[0].id }
			});

			return { conversation, workflows, workflowSteps, workflowStats, report };
		} catch (e) {
			notifications.addFlash({ message: 'Something went wrong', priority: 'ERROR' });
		}
	} catch (e) {
		notifications.addFlash({ message: 'No such report', priority: 'WARNING' });
		redirect(302, '/');
	}
};
