import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent }) => {
	const { api } = await parent();

	try {
		const conversations = await api.GetOwnedConversations()
		// TODO should be doing this on the server but Open API is not picking up
		// the parameters just yet 
		conversations.records = conversations.records.sort((a, b) => a.createdAt > b.createdAt ? 1 : -1)
		return { conversations }
	}

	catch (e) {
		if (e.status === 401) {
			notifications.addFlash({ message: "You are not authorised", priority: "WARNING" })
			redirect(302, "/")
		}
	}

}
