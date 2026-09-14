import type { PageLoad } from './$types';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { conversation_url } from '$lib/urls';

export type RoomDisplayMode = 'live' | 'demo';

/**
 * The Room display for one Polis step (CONTEXT.md, "Room display").
 *
 * Configuration is URL parameters, so any variation is a link you can send someone:
 *
 *   ?mode=demo         scripted scenario with animated joins and votes, for showing
 *                      the thing off; the default polls the real step
 *   ?variant=deck      the Deck direction instead of the console
 *   ?join=<url>        where the QR code points; defaults to the conversation page
 *   ?question=<text>   override the heading (the Polis topic by default)
 *   ?rate=<n>          demo only: playback speed
 *
 * Public, like `PolisGetReportData` itself: a projector or booth screen should not
 * need a login session. The conversation and step are fetched to find the Polis topic
 * and are optional in demo mode, so the demo also runs against ids that do not exist.
 */
export const load: PageLoad = async ({ parent, params, url, depends }) => {
	depends('app:room-display');
	const { api } = await parent();
	const { conversation_id, workflow_step_id } = params;
	const mode: RoomDisplayMode = url.searchParams.get('mode') === 'demo' ? 'demo' : 'live';

	const conversation = await tryCatchAsync(() =>
		api.GetConversation({ params: { conversation_id } })
	);
	const workflows =
		conversation.err === null
			? await tryCatchAsync(() =>
					api.ListConversationWorkflows({ params: { conversation_id } })
				)
			: null;
	// Steps hang off workflows, so every workflow is listed and the step found by id. A
	// conversation has one workflow in every case shipped today; this stays correct if
	// it ever has two.
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

	// A live conversation reads its published config; a draft one its preview config,
	// so a rehearsal before launch shows the topic the room will see.
	const toolConfig =
		step && (conversation.err === null && conversation.ok.isLive ? step.toolConfig : null);
	const config = toolConfig ?? step?.previewToolConfig ?? null;
	const topic = config?.type === 'polis' ? config.topic : null;

	return {
		mode,
		workflowStepId: workflow_step_id,
		question:
			url.searchParams.get('question') ??
			topic ??
			step?.name ??
			(mode === 'demo' ? 'What has to change for the Arctic over the next decade?' : 'Polis'),
		joinUrl:
			url.searchParams.get('join') ?? `${url.origin}${conversation_url(conversation_id)}`,
		variant: url.searchParams.get('variant') === 'deck' ? 'deck' : 'console',
		rate: Number(url.searchParams.get('rate')) || 90
	};
};
