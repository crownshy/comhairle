import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { RequestHandler } from './$types';

// Moderation moved to the Engage Moderate page. Keeps old bookmarks working.
export const GET: RequestHandler = ({ params }) => {
	redirect(
		308,
		resolve('/(admin)/admin/conversations/[conversation_id]/moderate/step/[step_id]', params)
	);
};
