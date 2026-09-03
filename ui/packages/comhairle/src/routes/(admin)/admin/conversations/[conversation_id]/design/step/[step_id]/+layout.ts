import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';
import { notifications } from '$lib/notifications.svelte';

/**
 * Shared shell for a single workflow step. Derives the step's sub-tab list from its
 * tool type (polis exposes Moderation/Insights; every other tool is Setup + Configure)
 * so the sub-tab strip can render from server data in `+layout.svelte`, instead of the
 * page injecting it post-hydration via an `$effect`. Setup (the tool work) leads; it's
 * the canonical landing and the most-visited tab.
 */
export const load: LayoutLoad = async ({ parent, params }) => {
	const { conversation, workflowSteps } = await parent();
	const { step_id, conversation_id } = params;

	const step = workflowSteps?.find((s) => s.id === step_id);

	if (!step) {
		notifications.addFlash({
			message: 'Could not find step',
			priority: 'ERROR'
		});
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/design', { conversation_id })
		);
	}

	const toolConfig = step
		? conversation.isLive
			? step.toolConfig
			: step.previewToolConfig
		: null;

	const subtabItems = [
		{ label: 'Setup', value: 'setup' },
		{ label: 'Configure', value: 'configure' }
	];

	// TODO: temporary until other tools include insights
	if (toolConfig?.type === 'polis') {
		subtabItems.push(
			{ label: 'Moderation', value: 'moderation' },
			{ label: 'Insights', value: 'insights' }
		);
	}
	if (
		toolConfig?.type === 'thinkingspace' ||
		toolConfig?.type === 'prioritization' ||
		toolConfig?.type === 'heyform'
	) {
		subtabItems.push({ label: 'Insights', value: 'insights' });
	}

	// `step` and `toolConfig` are shared by every sub-tab page (Configure/Setup/Moderation/
	// Insights), so they resolve once here and each page reads them from merged layout data.
	return { step, toolConfig, subtabItems };
};
