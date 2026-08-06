import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, data, depends }) => {
	depends('admin:organizations');
	const { api } = await parent();

	try {
		const ownedConversations = await api
			.GetOwnedConversations()
			.catch(() => ({ records: [], total: 0 }));

		const permittedConversations = await api
			.GetPermittedConversations({
				queries: { role_name: 'content_editor' }
			})
			.catch(() => ({ records: [], total: 0 }));

		const userOrganizations = await api.GetUserOrganizations().catch(() => ({
			organizations: [],
			canCreateOrganization: false
		}));

		// Forward the server-read sidebar width so the component's `data` carries it:
		// with a universal load present, SvelteKit does not auto-merge server data.
		return {
			ownedConversations,
			permittedConversations,
			userOrganizations,
			sidebarWidth: data.sidebarWidth
		};
	} catch (e) {
		if (e.status === 401) {
			notifications.addFlash({ message: 'You are not authorised', priority: 'WARNING' });
			redirect(302, '/');
		}
	}
};
