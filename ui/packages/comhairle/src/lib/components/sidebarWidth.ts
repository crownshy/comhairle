/**
 * Pure constants and helpers for the resizable admin sidebar width.
 *
 * The width is persisted in the `sidebar:width` cookie (not localStorage) so the
 * server can render the correct `--sidebar-width` on the first byte and the sidebar
 * never jumps on refresh. See ADR-0004 and {@link parseWidthCookie}.
 */

/** Fallback width (px) when no cookie is present. Matches shadcn's `18rem` default. */
export const DEFAULT_WIDTH = 288;
/** Smallest width (px) a drag can settle on before collapsing. */
export const MIN_WIDTH = 240;
/** Largest width (px) a drag can settle on. */
export const MAX_WIDTH = 480;
/** Drag position (px) below which the sidebar collapses to the icon rail. */
export const COLLAPSE_THRESHOLD = 100;
/** Width (px) a collapsed sidebar expands to when re-opened by click. */
export const EXPAND_WIDTH = 320;

/** Cookie name for the persisted width. Sits alongside shadcn's `sidebar:state`. */
export const WIDTH_COOKIE_NAME = 'sidebar:width';
/** Cookie lifetime (seconds): 7 days, mirroring `SIDEBAR_COOKIE_MAX_AGE`. */
const WIDTH_COOKIE_MAX_AGE = 60 * 60 * 24 * 7;

/** Clamp a candidate width to the allowed resize range `[MIN_WIDTH, MAX_WIDTH]`. */
export function clampWidth(px: number): number {
	return Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, px));
}

/**
 * Parse a raw `sidebar:width` cookie value into a usable width.
 *
 * Re-clamps so a tampered or stale cookie can never produce an out-of-range width.
 * Falls back to {@link DEFAULT_WIDTH} for a missing or non-numeric value.
 *
 * @param raw The cookie value (or `null`/`undefined` when absent).
 */
export function parseWidthCookie(raw: string | undefined | null): number {
	if (raw == null) return DEFAULT_WIDTH;
	const parsed = Number(raw);
	return Number.isFinite(parsed) ? clampWidth(parsed) : DEFAULT_WIDTH;
}

/**
 * Build the `document.cookie` string that persists `width`.
 *
 * The width is clamped before serialising so we never write an out-of-range value.
 *
 * @param width The width (px) to persist.
 */
export function serializeWidthCookie(width: number): string {
	return `${WIDTH_COOKIE_NAME}=${clampWidth(width)}; path=/; max-age=${WIDTH_COOKIE_MAX_AGE}; SameSite=Lax`;
}
