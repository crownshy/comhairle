import type { LayoutLoad } from './$types';
import type {
	ComhairleDocument,
	LocalizedWorkflowStepDto,
	LocalizedWorkflowStepWithProgressDto
} from '@crownshy/api-client/api';

export const load: LayoutLoad = async ({ parent, params, depends }) => {
	const { api, conversation, preview } = await parent();
	const workflow_id = params.workflow_id;

	depends('app:workflow-steps');
	depends('app:documents');

	// Documents power both the Learning Assistant gate and the in-content source badges.
	// Hoisted here (the nearest shared ancestor of the step page and the support sidebar) so a
	// single fetch is the one source of truth: the assistant is only shown when the knowledge
	// base has at least one parsed (DONE) document. A failed fetch falls back to "no documents",
	// which safely hides the assistant rather than surfacing a raw backend error to participants.
	let availableDocuments: ComhairleDocument[] = [];
	try {
		const documents = await api.ListDocuments({
			params: { conversation_id: conversation.id }
		});
		availableDocuments = documents.filter((d: ComhairleDocument) => d.parse_status === 'DONE');
	} catch (e) {
		console.warn('failed to load knowledge base documents', e);
	}
	const hasKnowledgeBaseDocs = availableDocuments.length > 0;

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
