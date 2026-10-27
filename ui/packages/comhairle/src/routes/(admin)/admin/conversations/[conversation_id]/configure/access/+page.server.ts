import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';

// Access used to be its own Configure tab. Old links land on its section of Details.
export const load: PageServerLoad = ({ params }) => {
	const { conversation_id } = params;
	const details = resolve('/(admin)/admin/conversations/[conversation_id]/configure/details', {
		conversation_id
	});

	redirect(HttpStatus.PermanentRedirect, `${details}#access-heading`);
};
