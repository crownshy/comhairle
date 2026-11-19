import { HttpStatus } from '$lib/utils/constants';
import { error, type ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = async ({ parent }) => {
	const { isSuperAdmin } = await parent();

	if (!isSuperAdmin) {
		error(HttpStatus.Forbidden, 'Super-admin access required');
	}
};
