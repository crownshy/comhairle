import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { thinkingSpaceInsightsLoader } from '$lib/reports/thinking-space/insights-loader';
import { polisInsightsLoader } from '$lib/reports/polis/insights-loader';

export const load: PageLoad = async (event) => {
	const { api, toolConfig, step_id } = await event.parent();

	// TODO: change to tool-agnostic
	event.depends('polis:statement-aux');
	event.depends('polis:report');

	try {
		// TODO: add other tool insights loaders
		if (toolConfig?.type === 'thinkingspace') {
			return await thinkingSpaceInsightsLoader(api, step_id);
		}

		if (toolConfig?.type === 'polis') {
			return await polisInsightsLoader(api, step_id);
		}

		redirect(
			307,
			`/admin/conversations/${event.params.conversation_id}/design/step/${event.params.step_id}/setup`
		);
	} catch (e) {
		console.error(e);
	}
};
