import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { RequestHandler } from './$types';

// Insights moved to the Engage Insights page. Keeps old bookmarks working.
export const GET: RequestHandler = ({ params }) => {
	redirect(
		308,
		resolve('/(admin)/admin/conversations/[conversation_id]/insights/step/[step_id]', params)
	);
};
