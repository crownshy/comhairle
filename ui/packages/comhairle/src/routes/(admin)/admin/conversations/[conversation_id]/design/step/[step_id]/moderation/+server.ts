import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Moderation moved to the Engage Moderate page. Keeps old bookmarks working.
export const GET: RequestHandler = ({ params }) => {
	redirect(308, `/admin/conversations/${params.conversation_id}/moderate/step/${params.step_id}`);
};
