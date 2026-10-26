const ADMIN_PREFIX = '/admin';

/**
 * Keep auth return targets as safe, same-origin paths and collapse accidental
 * /admin/admin prefixes into the canonical /admin path.
 */
export function normaliseBackTo(backTo: string | null, fallback = '/'): string {
	if (!backTo) return fallback;
	if (!backTo.startsWith('/') || backTo.startsWith('//') || backTo.includes('\\')) {
		return fallback;
	}

	return backTo.replace(
		new RegExp(`^(${ADMIN_PREFIX})+${ADMIN_PREFIX}(?=(/|\\?|#|$))`),
		ADMIN_PREFIX
	);
}
