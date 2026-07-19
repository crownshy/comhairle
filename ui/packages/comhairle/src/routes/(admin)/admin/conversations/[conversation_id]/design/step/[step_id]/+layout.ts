import type { LayoutLoad } from './$types';

/**
 * Shared shell for a single workflow step. Derives the step's sub-tab list from its
 * tool type (polis exposes Moderation/Insights; every other tool is Configure + Setup)
 * so the sub-tab strip can render from server data in `+layout.svelte`, instead of the
 * page injecting it post-hydration via an `$effect`.
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

	const subtabItems =
		toolConfig?.type === 'polis'
			? [
					{ label: 'Configure', value: 'configure' },
					{ label: 'Setup', value: 'setup' },
					{ label: 'Moderation', value: 'moderation' },
					{ label: 'Insights', value: 'insights' }
				]
			: [
					{ label: 'Configure', value: 'configure' },
					{ label: 'Setup', value: 'setup' }
				];

	return { step_id, subtabItems };
};
