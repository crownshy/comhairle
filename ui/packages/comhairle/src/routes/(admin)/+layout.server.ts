import type { LayoutServerLoad } from './$types';
import { parseWidthCookie, WIDTH_COOKIE_NAME } from '$lib/components/sidebarWidth';
import { error, redirect } from '@sveltejs/kit';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { canAccessAdminPortal } from '$lib/utils/permissions';
import { adminReturnPath } from './adminAccess';
import { HttpStatus } from '$lib/utils/constants';

/**
 * Read the persisted sidebar width from its cookie so the layout renders the correct
 * `--sidebar-width` on the first byte and the sidebar never jumps on refresh (ADR-0004).
 * The value is re-clamped in {@link parseWidthCookie}, so a tampered cookie is harmless.
 */
export const load: LayoutServerLoad = async ({ cookies, parent, locals, request, url }) => {
	const { user } = await parent();
	const returnPath = adminReturnPath(request.headers.get('referer'), url);
	if (!user) redirect(HttpStatus.SeeOther, returnPath);

	const roles = await tryCatchAsync(() => locals.api.GetUserRoles());
	if (roles.err !== null) {
		error(HttpStatus.InternalServerError, 'Unable to verify admin portal access.');
	}
	if (!canAccessAdminPortal(roles.ok)) {
		redirect(HttpStatus.SeeOther, returnPath);
	}

	return { sidebarWidth: parseWidthCookie(cookies.get(WIDTH_COOKIE_NAME)) };
};
