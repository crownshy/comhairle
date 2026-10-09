import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type {
	ConversationWithTranslations,
	MediaDto,
	UserWithPermissionDto,
	WorkflowStats,
	WorkflowStepsListResponse
} from '@crownshy/api-client/api';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { canPerformAction, loadUserActions } from '$lib/utils/permissions';

// Events load here, in parallel with the workflow fetch, so the layout can server-render the
// events strip from `data.events` the same way it renders the step strip.
export const load: LayoutLoad = async ({ params, parent, depends }) => {
	depends(key('admin/conversation'));
	depends(key('admin/conversation/workflow'));
	depends(key('admin/conversation/events'));
	depends(key('admin/conversation/meta'));
	depends('conversation:workflow');
	depends('conversation:events');
	depends('conversation:moderation-policy');

	const conversation_id = params.conversation_id;
	const { user, api } = await parent();

	try {
		const conversation = (await api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})) as ConversationWithTranslations;
		const conversationActions = await loadUserActions(api, 'conversation', conversation.id);
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

		const configureTabs: { id: string; label: string }[] = [
			{ id: 'details', label: 'Details' },
			{ id: 'content', label: 'Content' },
			{ id: 'glossary', label: 'Glossary' },
			{ id: 'moderation', label: 'Moderation policy' },
			{ id: 'access', label: 'Access' }
		];

		let usersWithPermission: UserWithPermissionDto[] = [];
		if (canPerformAction(conversationActions, 'list_permission')) {
			configureTabs.push({ id: 'team', label: 'Team' });
			usersWithPermission = await api.ListUsersWithPermission({
				params: {
					resource_type: 'conversation',
					resource_id: conversation.id
				},
				queries: { role_name: 'content_editor' }
			});
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
			events,
			media,
			user,
			cohostOrganizations,
			usersWithPermission,
			configureTabs,
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
