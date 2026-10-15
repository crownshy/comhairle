import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import {
	LATEST_CONVERSATION_COUNT,
	latestConversations,
	loadAdminConversations
} from './adminConversations';

export const load: LayoutLoad = async ({ parent, data, depends }) => {
	depends(key('admin/organizations'));
	depends(key('admin/conversations'));
	const { api, isSuperAdmin } = await parent();

	try {
		const [conversations, userOrganizations] = await Promise.all([
			loadAdminConversations(api, LATEST_CONVERSATION_COUNT),
			api.GetUserOrganizations().catch(() => ({
				organizations: [],
				canCreateOrganization: false
			}))
		]);

		// Forward the server-read sidebar width so the component's `data` carries it:
		// with a universal load present, SvelteKit does not auto-merge server data.
		return {
			latestConversations: latestConversations(conversations),
			userOrganizations,
			isSuperAdmin,
			sidebarWidth: data.sidebarWidth
		};
	} catch (e) {
		if (e.status === 401) {
			notifications.addFlash({ message: 'You are not authorised', priority: 'WARNING' });
			redirect(302, '/');
		}
	}
};
