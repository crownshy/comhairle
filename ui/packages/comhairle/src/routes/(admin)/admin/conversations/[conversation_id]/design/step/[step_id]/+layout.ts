import type { LayoutLoad } from './$types';

/**
 * Shared shell for a single workflow step. The sub-tab strip renders from this data in
 * `+layout.svelte`. Setup (the tool work) leads; it's the canonical landing and the
 * most-visited tab. Insights live on the Engage Insights page.
 */
export const load: LayoutLoad = async (event) => {
	const step_id = event.params.step_id;
	const { conversation, workflowSteps } = await event.parent();

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

	// `step` and `toolConfig` are shared by every sub-tab page, so they resolve once here and
	// each page reads them from merged layout data.
	return { step_id, step, toolConfig, subtabItems };
};
