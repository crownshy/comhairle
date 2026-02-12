import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { api, user } = await parent();
	if (!user) {
		redirect(307, '/');
	}

	// Fetch both unread notifications and unread count
	const [unreadNotifications, unreadCount, allNotifications] = await Promise.all([
		api.GetUnreadNotifications(),
		api.GetUnreadNotificationsCount(),
		api.GetAllNotifications()
	]);

	return {
		unreadNotifications,
		allNotifications,
		unreadCount
	};
};
