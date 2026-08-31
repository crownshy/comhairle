import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';

export const load: PageServerLoad = ({ params }) => {
	redirect(
		HttpStatus.PermanentRedirect,
		resolve('/(admin)/admin/conversations/[conversation_id]/invites/email', {
			conversation_id: params.conversation_id
		})
	);
};
