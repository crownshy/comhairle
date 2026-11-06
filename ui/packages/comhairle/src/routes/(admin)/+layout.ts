import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { ApiError, UserOrganizationsResponse } from '@crownshy/api-client/api';
import { HttpStatus } from '$lib/utils/constants';
import { typed } from '$lib/utils/types';

export const load: LayoutLoad = async ({ parent, data, depends }) => {
	depends(key('admin/organizations'));
	const { api } = await parent();

	const userOrganisations = await tryCatchAsync<UserOrganizationsResponse, ApiError>(() =>
		api.GetUserOrganizations()
	);
	if (userOrganisations.err !== null) {
		if (userOrganisations.err.status === HttpStatus.Unauthorized) {
			notifications.addFlash({ message: 'You are not authorised', priority: 'WARNING' });
			redirect(HttpStatus.Found, '/');
		}

		return {
			userOrganisations: typed<
				Pick<UserOrganizationsResponse, 'organizations' | 'canCreateOrganization'>
			>({ organizations: [], canCreateOrganization: false }),
			sidebarWidth: data.sidebarWidth
		};
	}

	// Forward the server-read sidebar width so the component's `data` carries it:
	// with a universal load present, SvelteKit does not auto-merge server data.
	return {
		userOrganisations: userOrganisations.ok,
		sidebarWidth: data.sidebarWidth
	};
};
