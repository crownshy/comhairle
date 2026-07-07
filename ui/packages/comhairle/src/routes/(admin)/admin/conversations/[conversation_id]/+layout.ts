import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type {
	ConversationWithTranslations,
	MediaDto,
	WorkflowDto,
	WorkflowStats,
	WorkflowStepWithTranslations
} from '@crownshy/api-client/api';
import type { LayoutLoad } from './$types';

/**
 * Invalidation keys for this load. Both re-run the same fetch, but the names
 * let callers express *what* they changed without coupling to load internals.
 * - conversation:meta — conversation record itself (title, description, flags…)
 * - conversation:workflow — workflows + steps + stats (anything step-related)
 */
export const load: LayoutLoad = async ({
	params,
	parent,
	depends
}): Promise<{
	conversation: ConversationWithTranslations;
	workflows: WorkflowDto[];
	workflowSteps: WorkflowStepWithTranslations[];
	stats: WorkflowStats;
	media: MediaDto | null;
}> => {
	depends('conversation:meta');
	depends('conversation:workflow');

	const conversation_id = params.conversation_id;
	const { api } = await parent();

	try {
		const conversation = (await api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})) as ConversationWithTranslations;
		const workflows = await api.ListConversationWorkflows({ params: { conversation_id } });
		let stats = undefined;
		let workflowSteps = undefined;

		let media: MediaDto | null = null;
		if (conversation.image) {
			media = await api.GetMedia({ params: { media_id: conversation.image } });
		}

		if (workflows.length > 0) {
			stats = await api.GetConversationWorkflowStats({
				params: { conversation_id, workflow_id: workflows[0].id }
			});
			workflowSteps = await api.ListConversationWorkflowSteps({
				params: { conversation_id, workflow_id: workflows[0].id },
				queries: { withTranslations: true }
			});
		}
		return { conversation, workflows, stats, workflowSteps, media };
	} catch (e) {
		console.error(e);
		notifications.addFlash({
			message: 'Problem loading conversation assets',
			priority: 'WARNING'
		});
		redirect(302, '/admin');
	}
};
