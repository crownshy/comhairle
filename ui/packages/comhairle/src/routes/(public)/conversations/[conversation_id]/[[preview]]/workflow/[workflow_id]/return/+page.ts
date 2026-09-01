import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { UserProgressDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, sealed, conversation, workflowSteps, preview, hasKnowledgeBaseDocs } =
		await parent();
	const { workflow_id } = params;

	let userProgress: UserProgressDto[] = [];
	const redirectUrl = `/conversations/${conversation.id}${preview ? '/preview' : ''}/workflow/${workflow_id}/next`;

	try {
		userProgress = await api.GetUserProgress({
			params: { conversation_id: conversation.id, workflow_id }
		});
	} catch (e) {
		console.error(e);

		return { error: e.response?.data?.err || 'Unable to load conversation assets' };
	}

	// This is the route behind the "come back to the conversation" links participants are
	// emailed, so it is the most likely way a sealed participant re-enters. `/next` sends them
	// on to the thank-you page.
	if (sealed) {
		redirect(302, redirectUrl);
	}

	if (userProgress.length === 0 || userProgress.some((progress) => progress.status !== 'done')) {
		redirect(302, redirectUrl);
	}

	const steps = [...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder);
	const revisitableSteps = steps.filter((step) => step.canRevisit);

	if (revisitableSteps.length === 0) {
		// No steps to revisit so send to thank you page via `/next`;
		redirect(302, redirectUrl);
	}

	return {
		conversation,
		steps,
		workflowId: workflow_id,
		preview,
		hasKnowledgeBaseDocs,
		// Same as the thank-you page: this screen carries the participant chrome, which has its
		// own preview marker, so the conversation layout's full-width banner stays off.
		participantChrome: true
	};
};
