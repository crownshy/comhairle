import type { Cookies } from '@sveltejs/kit';

const GRANT_COOKIE_PREFIX = 'preview-access-';
const PASSWORD_REQUIRED = 'Preview password required';

/**
 * The API sets one HttpOnly cookie per unlocked preview. Server-side API calls only send the
 * cookies we hand them, so these have to be passed along or a reload bounces the visitor back
 * to the password form.
 */
export function previewGrantCookies(cookies: Pick<Cookies, 'getAll'>): string[] {
	return cookies
		.getAll()
		.filter((cookie) => cookie.name.startsWith(GRANT_COOKIE_PREFIX))
		.map((cookie) => `${cookie.name}=${cookie.value}`);
}

type ApiErrorShape = { response?: { status?: number; data?: { err?: unknown } } };

export function isPreviewPasswordRequired(error: unknown): boolean {
	const response = (error as ApiErrorShape | null)?.response;
	return response?.status === 401 && response.data?.err === PASSWORD_REQUIRED;
}

export function previewUnlockUrl(conversationId: string, backTo: string): string {
	return `/conversations/${conversationId}/preview-unlock?backTo=${encodeURIComponent(backTo)}`;
}
