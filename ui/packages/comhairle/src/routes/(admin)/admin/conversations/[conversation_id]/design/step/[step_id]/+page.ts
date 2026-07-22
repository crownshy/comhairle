import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

/** Valid sub-tab route segments. A legacy `?subtab=` value is honoured if it matches one. */
const SUBTAB_SEGMENTS = ['configure', 'setup', 'moderation', 'insights'];

/**
 * The step index has no content of its own; it lands on the Setup sub-tab (the tool work,
 * the most-visited tab). Old `?subtab=` deep links redirect to the matching route so existing
 * bookmarks keep working. (A `moderation`/`insights` target on a non-polis step is bounced on
 * to Setup by that route's own guard.)
 */
export const load: PageLoad = async (event) => {
	const legacy = event.url.searchParams.get('subtab');
	const target = legacy && SUBTAB_SEGMENTS.includes(legacy) ? legacy : 'setup';
	redirect(
		307,
		`/admin/conversations/${event.params.conversation_id}/design/step/${event.params.step_id}/${target}`
	);
};
