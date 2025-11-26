/**
 * URL validation utilities for secure content embedding - maybe worth using a library here instead? Thoughts?
 */

const MAX_URL_LENGTH = 2048;

/**
 * @example
 * validateUrl('https://example.com/image.jpg') // Returns 'https://example.com/image.jpg'
 * validateUrl('javascript:alert(1)') // Returns null
 * validateUrl('http://example.com') // Returns null (not HTTPS)
 * validateUrl('https://...' + 'x'.repeat(3000)) // Returns null (too long)
 */
export function validateUrl(url: string | null | undefined): string | null {
	if (!url || typeof url !== 'string') {
		return null;
	}

	const trimmed = url.trim();

	if (trimmed.length > MAX_URL_LENGTH) {
		console.warn('[URL Validation] Blocked excessively long URL:', trimmed.length, 'characters');
		return null;
	}
	const dangerousProtocols = ['javascript:', 'data:', 'file:', 'vbscript:'];
	const lowerUrl = trimmed.toLowerCase();

	if (dangerousProtocols.some((protocol) => lowerUrl.startsWith(protocol))) {
		console.warn('[URL Validation] Blocked dangerous protocol:', trimmed);
		return null;
	}

	// Do we want to enforce https?
	if (!trimmed.startsWith('https://')) {
        // how do we want to log? I know console.warn isn't good practice
		console.warn('[URL Validation] Blocked non-HTTPS URL:', trimmed);
		return null;
	}

	try {
		new URL(trimmed);
		return trimmed;
	} catch {
        // 2  how do we want to log? I know console.warn isn't good practice
		console.warn('[URL Validation] Invalid URL format:', trimmed);
		return null;
	}
}

/**
 * Stricter validation with domain whitelist
 * 
 * @example
 * validateIframeUrl('https://youtube.com/embed/abc', ['youtube.com']) // Valid
 * validateIframeUrl('https://evil.com/embed/abc', ['youtube.com']) // Returns null
 * validateIframeUrl('https://youtube.com/embed/abc', []) // Returns null (no domains allowed)
 */
export function validateIframeUrl(
	url: string | null | undefined,
	allowedDomains: string[] = []
): string | null {
	const validUrl = validateUrl(url);
	if (!validUrl) {
		return null;
	}

	if (!allowedDomains || allowedDomains.length === 0) {
       // 3 how do we want to log? I know console.warn isn't good practice
		console.warn('[Iframe Validation] No allowed domains configured - blocking iframe');
		return null;
	}

	try {
		const urlObj = new URL(validUrl);
		const isAllowed = allowedDomains.some((domain) => {
			// Support wildcards like *.youtube.com
			if (domain.startsWith('*.')) {
				const baseDomain = domain.slice(2);
				return (
					urlObj.hostname === baseDomain || urlObj.hostname.endsWith(`.${baseDomain}`)
				);
			}
			return urlObj.hostname === domain;
		});

		if (!isAllowed) {
			console.warn(
				'[Iframe Validation] Blocked URL from non-whitelisted domain:',
				urlObj.hostname
			);
			return null;
		}

		return validUrl;
	} catch (error) {
		console.warn('[Iframe Validation] Failed to parse URL:', error);
		return null;
	}
}

export const DEFAULT_ALLOWED_DOMAINS = [
	'youtube.com',
	'*.youtube.com',
	'youtu.be',
	'vimeo.com',
	'player.vimeo.com'
];
