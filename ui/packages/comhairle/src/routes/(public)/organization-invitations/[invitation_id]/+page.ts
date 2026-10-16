import type { PageLoad } from './$types';
import type { ApiError, InvitationDetails } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';

export const load: PageLoad = async ({ params, parent }) => {
	const { api, user } = await parent();
	const invitationId = params.invitation_id;
	if (!user) return { user: null, invitationId, invitation: null, notFound: false };

	const result = await tryCatchAsync<InvitationDetails, ApiError>(() =>
		api.GetOrganizationConversationInvitation({ params: { invitation_id: invitationId } })
	);
	if (result.err !== null) {
		return { user, invitationId, invitation: null, notFound: true };
	}
	return { user, invitationId, invitation: result.ok, notFound: false };
};
