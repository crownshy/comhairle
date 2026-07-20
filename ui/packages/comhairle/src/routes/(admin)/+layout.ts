import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, data }) => {
	const { api } = await parent();

	try {
		const ownedConversations = await api.GetOwnedConversations();

		const permittedConversations = await api.GetPermittedConversations({
			queries: { role_name: 'content_editor' }
		});

		// Forward the server-read sidebar width so the component's `data` carries it:
		// with a universal load present, SvelteKit does not auto-merge server data.
		return { ownedConversations, permittedConversations, sidebarWidth: data.sidebarWidth };
	} catch (e) {
		if (e.status === 401) {
			notifications.addFlash({ message: 'You are not authorised', priority: 'WARNING' });
			redirect(302, '/');
		}
	}
};
