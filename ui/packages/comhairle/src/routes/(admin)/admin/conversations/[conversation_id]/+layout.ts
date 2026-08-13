import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type {
	ConversationWithTranslations,
	MediaDto,
	UserWithPermissionDto,
	WorkflowStats,
	WorkflowStepsListResponse
} from '@crownshy/api-client/api';
import type {} from '@crownshy/api-client/api';
import type { LayoutLoad } from './$types';

/**
 * Invalidation keys for this load. Each re-runs this fetch; the names let callers
 * express *what* they changed without coupling to load internals.
 * - conversation:meta — conversation record itself (title, description, flags…)
 * - conversation:workflow — workflows + steps + stats (anything step-related)
 * - conversation:events — the events list (create/rename/delete an event)
 *
 * Events are loaded here (not lazily in events/+layout) so the conversation layout can
 * server-render the events sub-tab strip from `data.events`, the same way it renders the
 * workflow step strip from `data.workflowSteps`. It runs in parallel with the workflow
 * fetch, so it adds no extra latency to a page load.
 */
export const load: LayoutLoad = async ({ params, parent, depends }) => {
	depends('conversation:meta');
	depends('conversation:workflow');
	depends('conversation:events');

	const conversation_id = params.conversation_id;
	const { user, api } = await parent();

	try {
		const conversation = (await api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})) as ConversationWithTranslations;
		const [workflows, eventsResponse, cohostOrganizations] = await Promise.all([
			api.ListConversationWorkflows({ params: { conversation_id } }),
			api.ListEvents({ params: { conversation_id }, queries: { created_at: 'desc' } }),
			api.ListConversationCoHostOrganizations({ params: { conversation_id } })
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
			{ id: 'access', label: 'Access' }
		];

		let usersWithPermission: UserWithPermissionDto[] = [];
		if (user.id === conversation.ownerId) {
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
			workflows,
			stats,
			workflowSteps,
			events,
			media,
			user,
			cohostOrganizations,
			usersWithPermission,
			configureTabs
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
