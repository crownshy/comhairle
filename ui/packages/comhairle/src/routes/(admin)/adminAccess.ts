import { resolve } from '$app/paths';
import type { UserRoles } from '@crownshy/api-client/api';
import { isModeratorOnly, moderatorOnlyConversationIds } from '$lib/utils/permissions';
import { pathWithQuery } from '$lib/utils/backTo';

export function adminLoginPath(currentUrl: URL): string {
	return `${resolve('/auth/login')}?backTo=${encodeURIComponent(pathWithQuery(currentUrl))}`;
}

export function adminReturnPath(referer: string | null, currentUrl: URL): string {
	if (!referer || !URL.canParse(referer)) return '/';
	const previousUrl = new URL(referer);
	if (
		previousUrl.origin !== currentUrl.origin ||
		previousUrl.pathname.startsWith('//') ||
		previousUrl.pathname.includes('\\') ||
		/^\/admin(?:\/|$)/.test(previousUrl.pathname)
	) {
		return '/';
	}
	return previousUrl.pathname + previousUrl.search + previousUrl.hash;
}

/**
 * Where `/admin` (and so login) sends you. Someone who only moderates one conversation goes
 * straight to its Moderate page; everyone else gets the conversations list (ADR-0065).
 */
export function adminLandingPath(roles: readonly UserRoles[] | null | undefined): string {
	const moderated = moderatorOnlyConversationIds(roles);
	if (isModeratorOnly(roles) && moderated.length === 1) {
		return `${resolve('/(admin)/admin/conversations/[conversation_id]', { conversation_id: moderated[0] })}/moderate`;
	}
	return resolve('/(admin)/admin/conversations');
}
