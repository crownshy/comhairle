import type { LayoutServerLoad } from './$types';
import { parseWidthCookie, WIDTH_COOKIE_NAME } from '$lib/components/sidebarWidth';

/**
 * Read the persisted sidebar width from its cookie so the layout renders the correct
 * `--sidebar-width` on the first byte and the sidebar never jumps on refresh (ADR-0004).
 * The value is re-clamped in {@link parseWidthCookie}, so a tampered cookie is harmless.
 */
export const load: LayoutServerLoad = async ({ cookies }) => {
	return { sidebarWidth: parseWidthCookie(cookies.get(WIDTH_COOKIE_NAME)) };
};
