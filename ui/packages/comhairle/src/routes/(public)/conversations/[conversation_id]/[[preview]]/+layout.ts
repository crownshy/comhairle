import { loginRedirect } from '$lib/urls';
import { isRedirect, redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import type { LocalizedConversationDto, WorkflowDto } from '@crownshy/api-client/api';

export const load: LayoutLoad = async ({
	parent,
	params
}): Promise<{
	conversation: LocalizedConversationDto;
	workflows: WorkflowDto[];
	participation: any; // TODO:
	api: any; // TODO:
	user: any; // TODO:
	preview: any; // TODO:
}> => {
	const { api, user } = await parent();
	const conversation_id = params.conversation_id;
	const preview = params.preview === 'preview';

	try {
		const conversation = await api.GetConversation({ params: { conversation_id } });

		if (!conversation.isLive && !preview) {
			return redirect(302, '/');
		}

		const workflows = await api.ListConversationWorkflows({
			params: { conversation_id: conversation.id }
		});

		let participation;

		if (user) {
			participation = await api.GetUserConversationParticipation({
				params: { conversation_id: conversation.id, workflow_id: workflows[0].id }
			});
		} else {
			participation = null;
		}

		return { conversation, workflows, participation, api, user, preview };
	} catch (e) {
		if (isRedirect(e)) {
			throw e;
		}
		return redirect(302, '/');
		loginRedirect(`/conversations/${params.conversation_id}`, 'Login to take part');
	}
};
