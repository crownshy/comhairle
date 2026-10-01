import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { conversation_url } from '$lib/urls';
import { key } from '$lib/utils/invalidationKey';
import { resolveBoard } from './board/blocks';
import { parseSurface } from './board/surfaces';

export type RoomDisplayMode = 'live' | 'demo';

/**
 * Loads the Room display for one Polis step. All settings are URL parameters (see NOTES.md).
 * The conversation and step are optional in demo mode, so the demo runs against any ids.
 */
export const load: PageLoad = async ({ parent, params, url, depends }) => {
	depends(key('public/conversation'), key('public/workflow-steps'));
	const { api } = await parent();
	const { conversation_id, workflow_step_id } = params;
	const mode: RoomDisplayMode = url.searchParams.get('mode') === 'demo' ? 'demo' : 'live';
	const boardParams = {
		preset: url.searchParams.get('variant'),
		layout: url.searchParams.get('layout'),
		blocks: url.searchParams.get('blocks'),
		latest: url.searchParams.get('latest'),
		theme: url.searchParams.get('theme'),
		scale: url.searchParams.get('scale'),
		sizes: url.searchParams.get('sizes'),
		slides: url.searchParams.get('slides')
	};

	const conversation = await tryCatchAsync(() =>
		api.GetConversation({ params: { conversation_id } })
	);
	const workflows =
		conversation.err === null
			? await tryCatchAsync(() =>
					api.ListConversationWorkflows({ params: { conversation_id } })
				)
			: null;
	// Steps belong to workflows, so search every workflow's steps for this id.
	const steps =
		workflows && workflows.err === null
			? await tryCatchAsync(() =>
					Promise.all(
						workflows.ok.map((w) =>
							api.ListConversationWorkflowSteps({
								params: { conversation_id, workflow_id: w.id }
							})
						)
					)
				)
			: null;
	const step =
		steps && steps.err === null
			? steps.ok.flat().find((s) => s.id === workflow_step_id)
			: undefined;

	// A draft conversation uses the preview config, so a rehearsal shows the real topic.
	const toolConfig =
		step && (conversation.err === null && conversation.ok.isLive ? step.toolConfig : null);
	const config = toolConfig ?? step?.previewToolConfig ?? null;
	const topic = config?.type === 'polis' ? config.topic : null;

	return {
		mode,
		workflowStepId: workflow_step_id,
		surface: parseSurface(url.searchParams.get('surface')),
		question:
			url.searchParams.get('question') ??
			topic ??
			step?.name ??
			(mode === 'demo' ? 'What has to change for the Arctic over the next decade?' : 'Polis'),
		joinUrl:
			url.searchParams.get('join') ?? `${url.origin}${conversation_url(conversation_id)}`,
		// Resolved again on the client once localStorage is readable. The raw parameters are
		// passed along so that pass can tell a parameter set in the URL from a missing one.
		boardParams,
		board: resolveBoard(boardParams),
		rate: Number(url.searchParams.get('rate')) || 90
	};
};
