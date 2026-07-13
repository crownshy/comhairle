import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent }) => {
	const { api } = await parent();

	try {
		const ownedConversations = await api.GetOwnedConversations();

		const permittedConversations = await api.GetPermittedConversations({
			queries: { role_name: 'content_editor' }
		});

		return { ownedConversations, permittedConversations };
	} catch (e) {
		if (e.status === 401) {
			notifications.addFlash({ message: 'You are not authorised', priority: 'WARNING' });
			redirect(302, '/');
		}
	}
};
