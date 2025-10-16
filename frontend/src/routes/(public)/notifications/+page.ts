import { redirect } from "@sveltejs/kit"
import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ parent }) => {
	let { api, user } = await parent()
	if (!user) {
		redirect(307, "/")
	}

	// Fetch both unread notifications and unread count
	const [unreadNotifications, unreadCount, allNotifications] = await Promise.all([
		api.getNotificationsunread(),
		api.getNotificationsunreadcount(),
		api.getNotifications()
	])

	return { 
		unreadNotifications,
		allNotifications, 
		unreadCount
	}
}
