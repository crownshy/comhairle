import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { ApiError, UserProgressDto } from '@crownshy/api-client/api';
import { next_workflow_step_url } from '$lib/urls';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { key } from '$lib/utils/invalidationKey';
import { revisitableSteps } from '../revisitableSteps';

export const load: PageLoad = async ({ parent, params, depends }) => {
	depends(key('public/user-progress'));
	const { api, conversation, workflowSteps, sealed } = await parent();
	const { workflow_id } = params;
	const isPreview = params.preview === 'preview';
	const nextUrl = next_workflow_step_url(conversation.id, workflow_id, isPreview);

	// This is the route behind the "come back to the conversation" links participants are
	// emailed, so it is the most likely way a sealed participant re-enters. `/next` sends them
	// on to the thank-you page.
	if (sealed) redirect(302, nextUrl);

	const progress = await tryCatchAsync<UserProgressDto[], ApiError>(() =>
		api.GetUserProgress({ params: { conversation_id: conversation.id, workflow_id } })
	);
	if (progress.err !== null) {
		return { loadFailed: true as const, conversation };
	}

	const hasFinished =
		progress.ok.length > 0 &&
		progress.ok.every((stepProgress) => stepProgress.status === 'done');
	const steps = revisitableSteps(workflowSteps);
	if (!hasFinished || steps.length === 0) redirect(302, nextUrl);

	return {
		loadFailed: false as const,
		conversation,
		revisitableSteps: steps,
		workflowId: workflow_id,
		isPreview
	};
};
