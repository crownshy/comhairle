import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { ConversationDto, UserProgressDto, WorkflowStepDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id, workflow_id, preview } = params;
	const isPreview = preview === 'preview';

	let userProgress: UserProgressDto[] = [];
	let conversation: ConversationDto;
	let redirectUrl = '/';
	const fallbackError = 'Unable to load conversation assets';

	try {
		// Get conversation first in case `conversation_id` params from url is
		// actually the conversation slug
		conversation = await api.GetConversation({ params: { conversation_id } });

		userProgress = await api.GetUserProgress({
			params: { conversation_id: conversation.id, workflow_id }
		});

		redirectUrl = `/conversations/${conversation.id}${isPreview ? '/preview' : ''}/workflow/${workflow_id}/next`;
	} catch (e) {
		console.error(e);

		return { error: e.response?.data?.err || fallbackError };
	}

	if (userProgress.length === 0 || userProgress.some((progress) => progress.status !== 'done')) {
		redirect(302, redirectUrl);
	} else {
		const workflowSteps: WorkflowStepDto[] = await api.ListConversationWorkflowSteps({
			params: { conversation_id: conversation.id, workflow_id }
		});
		const revisitableSteps = workflowSteps.filter((step) => step.canRevisit);

		if (revisitableSteps.length === 0) {
			// No steps to revisit so send to thank you page via `/next`;
			redirect(302, redirectUrl);
		}

		return { revisitableSteps, conversation, workflowId: workflow_id };
	}
};
