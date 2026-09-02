import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { PageLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

export const load: PageLoad = ({ params }) => {
	const { conversation_id, event_id } = params;
	redirect(
		HttpStatus.Found,
		resolve('/(admin)/admin/conversations/[conversation_id]/events/[event_id]/details', {
			conversation_id,
			event_id
		})
	);
};
