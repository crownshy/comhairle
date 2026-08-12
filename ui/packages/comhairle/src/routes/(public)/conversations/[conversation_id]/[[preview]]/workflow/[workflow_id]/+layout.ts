import type { LayoutLoad } from './$types';
import type {
	LocalizedWorkflowStepDto,
	LocalizedWorkflowStepWithProgressDto
} from '@crownshy/api-client/api';

export const load: LayoutLoad = async ({ parent, params, depends }) => {
	// Documents (the Learning Assistant gate + in-content source badges) are fetched once in the
	// conversation [[preview]] layout, the nearest shared ancestor of every page that renders
	// participant-facing rich content. Read them from there rather than re-fetching.
	const { api, conversation, preview, availableDocuments, hasKnowledgeBaseDocs } = await parent();
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

	return { workflowSteps, workflow_id, preview, availableDocuments, hasKnowledgeBaseDocs };
};
