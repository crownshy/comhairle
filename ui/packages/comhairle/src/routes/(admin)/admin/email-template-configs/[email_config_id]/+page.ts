import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ params, parent, depends }) => {
	depends(key('admin/email-template-config'));
	const { api } = await parent();
	const { email_config_id } = params;

	const result = await tryCatchAsync(() =>
		Promise.all([
			api.GetEmailTemplateConfig({ params: { email_config_id } }),
			api.GetEmailTemplateSchema({ params: { email_config_id } })
		])
	);
	if (result.err !== null) redirect(302, '/admin/email-template-configs');

	const [emailConfig, schema] = result.ok;

	// The conversation's banner for the preview. A failed lookup keeps the example banner.
	let conversationImageUrl: string | null = null;
	if (emailConfig.conversationId) {
		const conversationId = emailConfig.conversationId;
		const conversation = await tryCatchAsync(() =>
			api.GetConversation({ params: { conversation_id: conversationId } })
		);
		if (conversation.err === null && 'imageUrl' in conversation.ok) {
			conversationImageUrl = conversation.ok.imageUrl;
		}
	}

	return { emailConfig, schema, conversationImageUrl };
};
