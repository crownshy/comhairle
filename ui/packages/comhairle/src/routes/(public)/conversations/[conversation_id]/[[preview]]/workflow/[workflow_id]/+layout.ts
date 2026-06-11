import type { LayoutLoad } from './$types';
import type {
	LocalizedWorkflowStepDto,
	LocalizedWorkflowStepWithProgressDto
} from '@crownshy/api-client/api';

export const load: LayoutLoad = async ({ parent, params, depends }) => {
	const { api, conversation, preview } = await parent();
	const workflow_id = params.workflow_id;

	depends('app:workflow-steps');

	let workflowSteps: LocalizedWorkflowStepWithProgressDto[];
	if (conversation.isLive) {
		workflowSteps = (await api.ListConversationWorkflowSteps({
			params: { conversation_id: conversation.id, workflow_id },
			queries: { withUserProgress: true }
		})) as LocalizedWorkflowStepWithProgressDto[];
	} else {
		const steps = (await api.ListConversationWorkflowSteps({
			params: { conversation_id: conversation.id, workflow_id }
		})) as LocalizedWorkflowStepDto[];
		workflowSteps = steps.map((s) => ({
			...s,
			progressStatus: 'not_started' as const
		}));
	}

	return { workflowSteps, workflow_id, preview };
};
