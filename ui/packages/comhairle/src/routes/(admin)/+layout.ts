import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutLoad = async ({ parent, data, depends }) => {
	depends('admin:organizations');
	depends(key('conversations'));
	const { api } = await parent();

	try {
		const ownedConversations = await api
			.GetOwnedConversations()
			.catch(() => ({ records: [], total: 0 }));

		const ownedConversationIds = new Set(
			ownedConversations.records.map((conversation) => conversation.id)
		);

		const permittedConversations = await api
			.GetPermittedConversations()
			.catch(() => ({ records: [], total: 0 }));

		const nonOwnedPermittedConversationRecords = permittedConversations.records.filter(
			(conversation) => !ownedConversationIds.has(conversation.id)
		);

		const nonOwnedPermittedConversations = {
			...permittedConversations,
			records: nonOwnedPermittedConversationRecords,
			total: nonOwnedPermittedConversationRecords.length
		};

		const userOrganizations = await api.GetUserOrganizations().catch(() => ({
			organizations: [],
			canCreateOrganization: false
		}));

		// Forward the server-read sidebar width so the component's `data` carries it:
		// with a universal load present, SvelteKit does not auto-merge server data.
		return {
			ownedConversations,
			permittedConversations: nonOwnedPermittedConversations,
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
