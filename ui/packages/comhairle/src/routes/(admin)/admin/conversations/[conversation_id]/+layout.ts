import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type {
	ConversationWithTranslations,
	MediaDto,
	WorkflowStats,
	WorkflowStepsListResponse
} from '@crownshy/api-client/api';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { createPermissions, loadUserActions } from '$lib/utils/permissions';
import {
	conversationSurface,
	sectionSegment,
	surfaceRedirect
} from '$lib/utils/conversationSurface';

// Events load here, in parallel with the workflow fetch, so the layout can server-render the
// events strip from `data.events` the same way it renders the step strip.
export const load: LayoutLoad = async ({ params, parent, depends, url, untrack }) => {
	depends(key('admin/conversation'));
	depends(key('admin/conversation/workflow'));
	depends(key('admin/conversation/events'));
	depends(key('admin/conversation/meta'));
	depends('conversation:workflow');
	depends('conversation:events');
	depends('conversation:moderation-policy');

	const conversation_id = params.conversation_id;
	const { user, api } = await parent();

	const conversationActions = await loadUserActions(api, 'conversation', conversation_id);
	const permissions = createPermissions(() => [conversationActions]);
	const surface = conversationSurface((action) =>
		permissions.can('conversation', action, conversation_id)
	);
	// Untracked so moving between sections doesn't rerun this whole load. The sidebar only
	// links a moderator to Moderate, so this catches typed URLs and the conversation cards.
	const pathname = untrack(() => url.pathname);
	const redirectTo = surfaceRedirect(surface, sectionSegment(pathname, conversation_id));
	if (redirectTo) redirect(302, `/admin/conversations/${conversation_id}/${redirectTo}`);

	try {
		const conversation = (await api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})) as ConversationWithTranslations;
		const [
			workflows,
			eventsResponse,
			cohostOrganizations,
			moderationPolicies,
			defaultRejectReasons
		] = await Promise.all([
			api.ListConversationWorkflows({ params: { conversation_id } }),
			api.ListEvents({ params: { conversation_id }, queries: { created_at: 'desc' } }),
			api.ListConversationCoHostOrganizations({ params: { conversation_id } }),
			api.ListConversationModerationPolicies({ params: { conversation_id } }),
			api.GetDefaultModerationPolicyReasons({ params: { conversation_id } })
		]);
		// ListEvents returns a paginated `{ records }` wrapper; expose the flat array.
		const events = eventsResponse.records;
		let stats: WorkflowStats = {
			signupStats: [],
			stepStats: [],
			totalUsers: 0
		};
		let workflowSteps: WorkflowStepsListResponse = [];

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

		return {
			conversation,
			conversationActions,
			workflows,
			stats,
			workflowSteps,
			// Moderation is for Polis steps, whether they are live or still in preview.
			hasPolisStep: workflowSteps.some(
				(step) =>
					step.toolConfig?.type === 'polis' || step.previewToolConfig?.type === 'polis'
			),
			events,
			media,
			user,
			cohostOrganizations,
			// Read here once: the Configure editor and the Moderation tab both use them.
			moderationPolicies,
			defaultRejectReasons
		};
	} catch (e) {
		console.error(e);
		notifications.addFlash({
			message: 'Problem loading conversation assets',
			priority: 'WARNING'
		});
		redirect(302, '/admin');
	}
};
