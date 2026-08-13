import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { thinkingSpaceInsightsLoader } from '$lib/reports/thinking-space/insights-loader';
import { polisInsightsLoader } from '$lib/reports/polis/insights-loader';
import { prioritizationInsightsLoader } from '$lib/reports/prioritization/insights-loader';
import { surveyInsightsLoader } from '$lib/reports/survey/insights-loader';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { HttpStatus } from '$lib/utils/constants';

export const load: PageLoad = async (event) => {
	const { api, toolConfig } = await event.parent();
	const { step_id } = event.params;

	// TODO: change to tool-agnostic
	event.depends('polis:statement-aux');
	event.depends('polis:report');

	const response = await tryCatchAsync(async () => {
		switch (toolConfig?.type) {
			case 'polis':
				return await polisInsightsLoader(api, step_id);
			case 'prioritization':
				return await prioritizationInsightsLoader(api, step_id);
			case 'thinkingspace':
				return await thinkingSpaceInsightsLoader(api, step_id);
			case 'heyform':
				return await surveyInsightsLoader(api, step_id);
			case 'learn':
			case 'stories':
			case 'elicitationbot':
				throw Error('Not yet implemented');
		}
	});

	if (response.err !== null) {
		redirect(
			HttpStatus.TemporaryRedirect,
			`/admin/conversations/${event.params.conversation_id}/design/step/${event.params.step_id}/setup`
		);
	}

	return response.ok;
};
