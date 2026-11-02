import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';

export const load: LayoutLoad = async ({ parent, data, depends }) => {
	depends(key('admin/organizations'));
	const { api, isSuperAdmin } = await parent();

	try {
		const userOrganizations = await api.GetUserOrganizations().catch(() => ({
			organizations: [],
			canCreateOrganization: false
		}));

		// Forward the server-read sidebar width so the component's `data` carries it:
		// with a universal load present, SvelteKit does not auto-merge server data.
		return {
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
