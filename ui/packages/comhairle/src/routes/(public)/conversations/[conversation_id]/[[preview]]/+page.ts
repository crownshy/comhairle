import type { PageLoad } from './$types';
import type { LocalizedWorkflowStepDto } from '@crownshy/api-client/api';

/**
 * The landing page shows what taking part involves, which needs the workflow's steps.
 *
 * Fetched here rather than in the conversation layout because the landing page is the only
 * page under that layout which needs them: the FAQ, privacy and report pages do not, and
 * the workflow layout fetches its own copy with per-user progress attached.
 *
 * A failed fetch is not fatal. The page falls back to a cover with no step list and no
 * estimate, which is what the page looked like before, rather than failing the route.
 */
export const load: PageLoad = async ({ parent, depends }) => {
	depends('app:workflow-steps');

	const { api, conversation, workflows } = await parent();

	if (!workflows?.length) {
		return { workflowSteps: [] as LocalizedWorkflowStepDto[], participantChrome: true };
	}

	try {
		const workflowSteps = (await api.ListConversationWorkflowSteps({
			params: { conversation_id: conversation.id, workflow_id: workflows[0].id }
		})) as LocalizedWorkflowStepDto[];
		return { workflowSteps, participantChrome: true };
	} catch (e) {
		console.warn('failed to load workflow steps for the conversation landing page', e);
		return { workflowSteps: [] as LocalizedWorkflowStepDto[], participantChrome: true };
	}
};
