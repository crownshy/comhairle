import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { ConversationWithTranslations, UserWithPermissionDto } from '@crownshy/api-client/api';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { HttpStatus } from '$lib/utils/constants';

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
	depends(key('conversation'));
	depends(key('conversation/workflow'));
	depends('conversation:meta');
	depends('conversation:workflow');
	depends('conversation:events');

	const { conversation_id } = params;
	const { user, api } = await parent();

	const conversationResponse = await tryCatchAsync(() =>
		api.GetConversation({
			params: { conversation_id },
			queries: { withTranslations: true }
		})
	);

	if (conversationResponse.err !== null) {
		console.error(conversationResponse.err);
		notifications.addFlash({
			message: 'Problem loading conversation assets',
			priority: 'WARNING'
		});
		redirect(HttpStatus.Found, '/admin');
	}

	const conversation = conversationResponse.ok as ConversationWithTranslations;

	const workflowsResponse = tryCatchAsync(() =>
		api.ListConversationWorkflows({ params: { conversation_id } })
	);

	const cohostOrganizationsResponse = tryCatchAsync(() =>
		api.ListConversationCoHostOrganizations({ params: { conversation_id } })
	);

	const workflows = await workflowsResponse;
	if (workflows.err !== null) {
		console.error(workflows.err);
		notifications.addFlash({
			message: 'Problem loading workflows',
			priority: 'WARNING'
		});
		redirect(HttpStatus.Found, '/admin');
	}

	const cohostOrganizations = await cohostOrganizationsResponse;
	if (cohostOrganizations.err !== null) {
		console.error(cohostOrganizations.err);
		notifications.addFlash({
			message: 'Problem loading workflows',
			priority: 'WARNING'
		});
		redirect(HttpStatus.Found, '/admin');
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

	return {
		conversation,
		user,
		workflows: workflows.ok,
		cohostOrganizations: cohostOrganizations.ok,
		usersWithPermission,
		configureTabs
	};
};
