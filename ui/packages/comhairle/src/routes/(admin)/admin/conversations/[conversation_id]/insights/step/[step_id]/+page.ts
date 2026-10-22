import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { PageLoad } from './$types';
import { thinkingSpaceInsightsLoader } from '$lib/reports/thinking-space/insights-loader';
import { polisInsightsLoader } from '$lib/reports/polis/insights-loader';
import { prioritizationInsightsLoader } from '$lib/reports/prioritization/insights-loader';
import { surveyInsightsLoader } from '$lib/reports/survey/insights-loader';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { HttpStatus } from '$lib/utils/constants';
import { key } from '$lib/utils/invalidationKey';

export const load: PageLoad = async (event) => {
	const { api, conversation, workflowSteps } = await event.parent();
	const { step_id } = event.params;

	// TODO: change to tool-agnostic
	event.depends(key('admin/polis/statements'));
	event.depends(key('admin/polis/report'));
	event.depends(key('admin/polis/summary'));

	const step = workflowSteps.find((s) => s.id === step_id);
	const toolConfig = step && (conversation.isLive ? step.toolConfig : step.previewToolConfig);

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
			default:
				throw Error('This step has no Insights');
		}
	});

	if (!step || response.err !== null) {
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/insights', {
				conversation_id: event.params.conversation_id
			})
		);
	}

	return { step, ...response.ok };
};
