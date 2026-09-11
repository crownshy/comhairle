import { tryCatchAsync } from '$lib/utils/errorHandling';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, streamedInvites } = await parent();
	const { conversation_id } = params;

	return {
		streamedStats: streamedInvites.then((invites) => {
			if (invites.err !== null) {
				return { ok: null, err: invites.err };
			}

			return {
				err: null,
				ok: Object.fromEntries(
					invites.ok.openInvites.map((openInvite) => [
						openInvite.id,
						tryCatchAsync(() =>
							api
								.GetInviteStats({
									params: { conversation_id, invite_id: openInvite.id }
								})
								.then((stats) =>
									stats.map((s) => ({
										...s,
										day: new Date(s.day).toLocaleDateString('en-US', {
											month: 'short',
											day: '2-digit'
										})
									}))
								)
						)
					])
				)
			};
		})
	};
};
