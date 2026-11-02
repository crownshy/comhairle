import { isRedirect, redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';
import type {
	ComhairleDocument,
	LocalizedConversationDto,
	UserParticipationDto,
	WorkflowDto
} from '@crownshy/api-client/api';
import type { createApiClient } from '@crownshy/api-client/client';
import { key } from '$lib/utils/invalidationKey';
import { isPreviewPasswordRequired, previewUnlockUrl } from '$lib/utils/previewAccess';

export const load: LayoutLoad = async ({
	parent,
	params,
	depends,
	url
}): Promise<{
	conversation: LocalizedConversationDto;
	workflows: WorkflowDto[];
	availableDocuments: ComhairleDocument[];
	hasKnowledgeBaseDocs: boolean;
	participation: UserParticipationDto | null;
	api: ReturnType<typeof createApiClient>;
	user: any; // TODO:
	preview: boolean;
}> => {
	// The participation row carries the seal, which the write that finishes the flow brings
	// into existence. Without its own key, invalidating after that write reruns the workflow
	// layout but not this one, so `sealed` here would stay false for the rest of the session
	// and browser Back would walk straight into a step. See ADR-0016.
	depends(key('public/participation'));
	depends(key('public/conversation'));
	depends(key('public/documents'));
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

		// Parsed knowledge base documents. Hoisted to this shared ancestor (the FAQ, privacy,
		// thank-you and landing pages, plus the workflow layout's Learning Assistant and in-content
		// source badges) so a single fetch is the one source of truth. `hasKnowledgeBaseDocs` gates
		// the Learning Assistant; the list resolves each source-document badge's name/size/download.
		// A failed fetch falls back to "no documents", which safely hides the assistant and renders
		// badges as bare labels rather than surfacing a raw backend error to participants.
		let availableDocuments: ComhairleDocument[] = [];
		try {
			const documents = await api.ListDocuments({
				params: { conversation_id: conversation.id }
			});
			availableDocuments = documents.filter(
				(d: ComhairleDocument) => d.parse_status === 'DONE'
			);
		} catch (e) {
			console.warn('failed to load knowledge base documents', e);
		}
		const hasKnowledgeBaseDocs = availableDocuments.length > 0;

		let participation;

		if (user) {
			participation = await api.GetUserConversationParticipation({
				params: { conversation_id: conversation.id, workflow_id: workflows[0].id }
			});
		} else {
			participation = null;
		}

		return {
			conversation,
			workflows,
			availableDocuments,
			hasKnowledgeBaseDocs,
			participation,
			api,
			user,
			preview
		};
	} catch (e) {
		if (isRedirect(e)) {
			throw e;
		}
		if (preview && isPreviewPasswordRequired(e)) {
			redirect(302, previewUnlockUrl(conversation_id, url.pathname + url.search));
		}
		return redirect(302, '/');
	}
};
