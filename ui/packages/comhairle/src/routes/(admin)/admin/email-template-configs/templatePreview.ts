import { apiClient } from '@crownshy/api-client/client';
import type { EmailTypeSchema } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { EmailTemplateRow } from '$lib/utils/emailTemplateGroups';
import {
	applyPreviewConversation,
	buildSlotPayload,
	fillKnownVariables,
	fillSlotVariables,
	initialSlotValues,
	type PreviewConversation
} from '$lib/utils/emailTemplateForm';

export type RenderedPreview = { subject: string; html: string };

/** Renders a saved version (or the default) as participants of that conversation receive it. */
export async function renderTemplatePreview(
	schema: EmailTypeSchema,
	row: EmailTemplateRow,
	conversation: PreviewConversation | null
): Promise<RenderedPreview | null> {
	const known = conversation ? { conversation_title: conversation.title } : {};
	const slots = buildSlotPayload(schema, initialSlotValues(schema, row.slots));
	const result = await tryCatchAsync(() =>
		apiClient.PreviewEmailTemplateConfig({
			slots: { type: schema.email_type, ...fillSlotVariables(schema, slots, known) }
		})
	);
	if (result.err !== null) return null;

	return {
		subject: fillKnownVariables(row.subject, known),
		html: applyPreviewConversation(result.ok.html, conversation)
	};
}
