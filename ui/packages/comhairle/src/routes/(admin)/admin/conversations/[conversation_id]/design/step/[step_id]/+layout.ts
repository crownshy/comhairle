import type { LayoutLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';

/**
 * Shared shell for a single workflow step. Derives the step's sub-tab list from its
 * tool type (polis exposes Moderation/Insights; every other tool is Setup + Configure)
 * so the sub-tab strip can render from server data in `+layout.svelte`, instead of the
 * page injecting it post-hydration via an `$effect`. Setup (the tool work) leads; it's
 * the canonical landing and the most-visited tab.
 */
export const load: LayoutLoad = async (event) => {
	// Source documents feed both the description editor's badges and the participant view
	// that renders those badges back, so they resolve once here rather than in each.
	// Same invalidation key the participant side uses for the same fetch.
	event.depends('app:documents');

	const step_id = event.params.step_id;
	const { api, conversation, workflowSteps } = await event.parent();

	const documents = await tryCatchAsync(() =>
		api.ListDocuments({ params: { conversation_id: event.params.conversation_id } })
	);
	const availableDocuments =
		documents.err === null ? documents.ok.filter((d) => d.parse_status === 'DONE') : [];

	const step = workflowSteps?.find((s) => s.id === step_id);
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
	return { step_id, step, toolConfig, subtabItems, availableDocuments };
};
