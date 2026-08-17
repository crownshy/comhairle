import type { LayoutLoad } from './$types';
import type {
	LocalizedWorkflowStepDto,
	LocalizedWorkflowStepWithProgressDto,
	UserParticipationDto
} from '@crownshy/api-client/api';

export const load: LayoutLoad = async ({ parent, params, depends }) => {
	// Documents (the Learning Assistant gate + in-content source badges) are fetched once in the
	// conversation [[preview]] layout, the nearest shared ancestor of every page that renders
	// participant-facing rich content. Read them from there rather than re-fetching.
	const { api, conversation, preview, participation, availableDocuments, hasKnowledgeBaseDocs } =
		await parent();
	const workflow_id = params.workflow_id;

	depends('app:workflow-steps');
	// This layout fetches participation itself when the parent's row is for another workflow,
	// so it needs the same key the conversation layout declares.
	depends('app:participation');

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

	// The conversation layout fetches participation for the conversation's first workflow,
	// which is the one this flow runs in every case we ship today. If this layout is ever
	// mounted on a second workflow, that row is for the wrong one and its seal cannot be
	// trusted, so fetch the row for this workflow instead. No row at all means the participant
	// has never started, which cannot be sealed, so there is nothing to fetch.
	const participationForWorkflow: UserParticipationDto | null =
		participation && participation.workflow_id !== workflow_id
			? await api.GetUserConversationParticipation({
					params: { conversation_id: conversation.id, workflow_id }
				})
			: participation;

	// Whether this participant is sealed out of the flow is decided by the backend (one
	// helper, shared with the write gates that enforce it - see ADR-0016). Hoisted here so the
	// step page, thank-you page and return route all read the same value: they each filter on
	// revisitability independently, and `/return` is the magic link participants are emailed,
	// so a disagreement between them is the one people would actually hit.
	//
	// Preview is exempt. The backend has no notion of previewing, so an admin previewing a
	// live conversation they happen to have participated in would otherwise be sealed out of
	// their own preview.
	const sealed = !preview && conversation.isLive && (participationForWorkflow?.sealed ?? false);

	return {
		workflowSteps,
		workflow_id,
		preview,
		sealed,
		availableDocuments,
		hasKnowledgeBaseDocs
	};
};
