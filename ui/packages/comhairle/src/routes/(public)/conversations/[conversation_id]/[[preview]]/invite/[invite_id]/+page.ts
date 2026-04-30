import { isRedirect, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { conversation_url } from '$lib/urls';
import type { InviteDto } from '@crownshy/api-client/api';

export const load: PageLoad = async ({ params, parent, url }) => {
	const { api, conversation, user, workflows, participation } = await parent();
	const { invite_id } = params;

	// Preserve query parameters for redirects
	const queryString = url.search;

	try {
		const invite: InviteDto = await api.GetInvite({
			params: { conversation_id: conversation.id, invite_id }
		});

		/* Event invites */
		if (invite.eventId) {
			if (user) {
				try {
					await api.CreateEventAttendance(
						{ role: 'participant' },
						{
							params: {
								conversation_id: conversation.id,
								event_id: invite.eventId
							}
						}
					);

					try {
						await api.AcceptInvite(undefined, {
							params: { conversation_id: conversation.id, invite_id: invite.id }
						});
					} catch (e) {
						throw new Error(e);
					}

					redirect(302, `/conversations/${conversation.id}/events/${invite.eventId}`);
				} catch (e) {
					// Log error but don't propagate as the user may already
					// be registered with event
					console.error(e);
				}
			} else {
				await api.AutoRegisterEventAttendance(undefined, {
					params: { conversation_id: conversation.id, invite_id: invite.id }
				});

				return redirect(302, `/conversations/${conversation.id}/events/${invite.eventId}`);
			}
		}

		// /* Conversation invites */
		if (!user && invite.loginBehaviour == 'auto_create_annon') {
			await api.SignupAnnonUser(undefined, {});
			redirect(307, conversation_url(conversation.id) + queryString);
		}
		if (user && invite.status === 'accepted') {
			return redirect(307, conversation_url(conversation.id) + queryString);
		}
		// Auto-redirect if user is already registered for the conversation
		if (user && participation) {
			const firstWorkflow = workflows[0];
			redirect(
				307,
				`/conversations/${conversation.id}/workflow/${firstWorkflow.id}/next${queryString}`
			);
		}
		return { invite, conversation, user, workflows, participation };
	} catch (e) {
		if (isRedirect(e)) {
			throw e;
		}
		console.error(e);
		return { error: e.response.data.err, conversation };
	}
};
