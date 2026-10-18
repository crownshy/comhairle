import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

/** Valid sub-tab route segments. A legacy `?subtab=` value is honoured if it matches one. */
const SUBTAB_SEGMENTS = ['configure', 'setup', 'insights'];

/**
 * The step index has no content of its own; it lands on the Setup sub-tab (the tool work,
 * the most-visited tab). Old `?subtab=` deep links redirect to the matching route so existing
 * bookmarks keep working. Moderation now lives on the Engage Moderate page.
 */
export const load: PageLoad = async (event) => {
	const legacy = event.url.searchParams.get('subtab');
	if (legacy === 'moderation') {
		redirect(
			307,
			`/admin/conversations/${event.params.conversation_id}/moderate/step/${event.params.step_id}`
		);
	}
	const target = legacy && SUBTAB_SEGMENTS.includes(legacy) ? legacy : 'setup';
	redirect(
		307,
		`/admin/conversations/${event.params.conversation_id}/design/step/${event.params.step_id}/${target}`
	);
};
