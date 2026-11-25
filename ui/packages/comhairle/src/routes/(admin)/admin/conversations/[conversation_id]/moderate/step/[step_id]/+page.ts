import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { key } from '$lib/utils/invalidationKey';
import { loadPolisStatements, loadStepTranslations } from '$lib/tools/polis/loadStatements';
import { isModeratableStep } from '../../moderatableSteps';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent, depends }) => {
	// Re-run after sync, moderate, split and seed.
	depends(key('admin/polis/statements'));
	const { api, conversation, workflowSteps } = await parent();

	const step = workflowSteps.find((s) => s.id === params.step_id);
	const toolConfig = step && (conversation.isLive ? step.toolConfig : step.previewToolConfig);
	if (!step || toolConfig?.type !== 'polis') {
		redirect(
			307,
			resolve('/(admin)/admin/conversations/[conversation_id]/moderate', {
				conversation_id: params.conversation_id
			})
		);
	}

	// With one step the list page redirects here, so a link back to it would loop.
	const hasOtherSteps =
		workflowSteps.filter((s) => isModeratableStep(conversation.isLive, s)).length > 1;

	const [statements, translations] = await Promise.all([
		loadPolisStatements(api, step.id),
		loadStepTranslations(api, step.id)
	]);
	return { step, toolConfig, hasOtherSteps, ...statements, ...translations };
};
