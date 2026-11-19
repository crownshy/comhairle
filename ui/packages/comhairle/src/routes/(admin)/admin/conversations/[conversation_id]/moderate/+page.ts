import { redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import { isModeratableStep } from './moderatableSteps';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { conversation, workflowSteps } = await parent();
	const polisSteps = workflowSteps.filter((step) => isModeratableStep(conversation.isLive, step));
	// A list of one is a click with no choice in it.
	if (polisSteps.length === 1) {
		redirect(
			307,
			resolve('/(admin)/admin/conversations/[conversation_id]/moderate/step/[step_id]', {
				conversation_id: params.conversation_id,
				step_id: polisSteps[0].id
			})
		);
	}
	return { polisSteps };
};
