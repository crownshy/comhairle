/**
 * URL validation utilities for secure content embedding.
 * Uses URL.parse() (returns null for invalid URLs) and Zod for schema validation.
 */

import { z } from 'zod';

const MAX_URL_LENGTH = 2048;
const DANGEROUS_PROTOCOLS = ['javascript:', 'data:', 'file:', 'vbscript:'];

export const urlSchema = z
	.string()
	.max(MAX_URL_LENGTH)
	.url()
	.refine((url) => url.startsWith('https://'), { message: 'URL must use HTTPS' })
	.refine(
		(url) => !DANGEROUS_PROTOCOLS.some((p) => url.toLowerCase().startsWith(p)),
		{ message: 'Dangerous protocol detected' }
	);

/**
 * Validates a URL string for security and format.
 * Uses URL.parse() which returns null for invalid URLs (no try/catch needed).
 *
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
		return null;
	}

	const lowerUrl = trimmed.toLowerCase();
	if (DANGEROUS_PROTOCOLS.some((protocol) => lowerUrl.startsWith(protocol))) {
		return null;
	}

	if (!trimmed.startsWith('https://')) {
		return null;
	}

	return URL.parse(trimmed) ? trimmed : null;
}

/**
 * Stricter validation with domain whitelist.
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
		return null;
	}

	const urlObj = URL.parse(validUrl);
	if (!urlObj) {
		return null;
	}

	const isAllowed = allowedDomains.some((domain) => {
		// Support wildcards like *.youtube.com
		if (domain.startsWith('*.')) {
			const baseDomain = domain.slice(2);
			return urlObj.hostname === baseDomain || urlObj.hostname.endsWith(`.${baseDomain}`);
		}
		return urlObj.hostname === domain;
	});

	return isAllowed ? validUrl : null;
}

export const DEFAULT_ALLOWED_DOMAINS = [
	'youtube.com',
	'*.youtube.com',
	'youtu.be',
	'vimeo.com',
	'player.vimeo.com'
];
