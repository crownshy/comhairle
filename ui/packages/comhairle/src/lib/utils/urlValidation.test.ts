import { describe, it, expect } from 'vitest';
import { validateUrl, validateIframeUrl, DEFAULT_ALLOWED_DOMAINS } from './urlValidation';

describe('validateUrl', () => {
	describe('valid HTTPS URLs', () => {
		it('should accept basic HTTPS URLs', () => {
			expect(validateUrl('https://example.com/image.jpg')).toBe('https://example.com/image.jpg');
			expect(validateUrl('https://cdn.example.com/photos/pic.png')).toBe('https://cdn.example.com/photos/pic.png');
		});

		it('should accept URLs with query parameters', () => {
			expect(validateUrl('https://images.unsplash.com/photo-123?w=800')).toBe('https://images.unsplash.com/photo-123?w=800');
			expect(validateUrl('https://example.com/page?query=test&foo=bar')).toBe('https://example.com/page?query=test&foo=bar');
		});

		it('should accept URLs with ports', () => {
            //QUESTION: DO we want this?
			expect(validateUrl('https://example.com:8443/secure')).toBe('https://example.com:8443/secure');
		});

		it('should accept URLs with subdomains', () => {
			expect(validateUrl('https://sub.domain.example.com/path')).toBe('https://sub.domain.example.com/path');
		});

		it('should accept URLs with fragments', () => {
			expect(validateUrl('https://example.com/page#section')).toBe('https://example.com/page#section');
		});

		it('should accept URLs with deep paths', () => {
			expect(validateUrl('https://example.com/a/b/c/d/e/f/g/image.jpg')).toBe('https://example.com/a/b/c/d/e/f/g/image.jpg');
		});
	});

	describe('dangerous protocols', () => {
		it('should block javascript: protocol', () => {
			expect(validateUrl('javascript:alert(1)')).toBeNull();
			expect(validateUrl('javascript:void(0)')).toBeNull();
			expect(validateUrl('JAVASCRIPT:alert(1)')).toBeNull(); // Case insensitive
		});

		it('should block data: protocol', () => {
			expect(validateUrl('data:text/html,<script>alert(1)</script>')).toBeNull();
			expect(validateUrl('data:image/svg+xml,<svg onload=alert(1)>')).toBeNull();
		});

		it('should block file: protocol', () => {
			expect(validateUrl('file:///etc/passwd')).toBeNull();
			expect(validateUrl('file:///C:/Windows/System32')).toBeNull();
		});

		it('should block vbscript: protocol', () => {
			expect(validateUrl('vbscript:msgbox(1)')).toBeNull();
		});
	});

	describe('non-HTTPS URLs', () => {
		it('should block HTTP URLs', () => {
			expect(validateUrl('http://example.com/image.jpg')).toBeNull();
			expect(validateUrl('http://www.youtube.com/embed/abc')).toBeNull();
		});

		it('should block FTP URLs', () => {
			expect(validateUrl('ftp://example.com/file.txt')).toBeNull();
		});

        // The following two maybe we don't want?
		it('should block protocol-relative URLs', () => {
			expect(validateUrl('//example.com/image.jpg')).toBeNull();
		});

		it('should block URLs without protocol', () => {
			expect(validateUrl('example.com/image.jpg')).toBeNull();
		});
	});

	describe('malformed URLs', () => {
		it('should block empty or whitespace strings', () => {
			expect(validateUrl('')).toBeNull();
			expect(validateUrl('   ')).toBeNull();
			expect(validateUrl('  \n  ')).toBeNull();
		});

		it('should block null and undefined', () => {
			expect(validateUrl(null)).toBeNull();
			expect(validateUrl(undefined)).toBeNull();
		});

		it('should block invalid URL formats', () => {
			expect(validateUrl('not a url at all')).toBeNull();
			expect(validateUrl('https://')).toBeNull();
			expect(validateUrl('https://invalid domain with spaces.com')).toBeNull();
			expect(validateUrl('ht tps://example.com')).toBeNull();
		});
	});

	describe('edge cases', () => {
		it('should trim whitespace from URLs', () => {
			expect(validateUrl('  https://example.com/image.jpg  ')).toBe('https://example.com/image.jpg');
			expect(validateUrl('\nhttps://example.com/image.jpg\n')).toBe('https://example.com/image.jpg');
		});

		it('should handle URLs with unicode characters', () => {
			// Unicode in path
			const unicodeUrl = validateUrl('https://example.com/café/image.jpg');
			expect(unicodeUrl).toBeTruthy();
			expect(unicodeUrl).toContain('example.com');
		});

		it('should handle non-string inputs', () => {
			expect(validateUrl(123 as any)).toBeNull();
			expect(validateUrl({} as any)).toBeNull();
			expect(validateUrl([] as any)).toBeNull();
		});
	});

	describe('URL length validation', () => {

		it('should accept URLs exactly at the 2048 character limit', () => {
			// 2048 characters
			const exactLimitUrl = 'https://example.com/' + 'x'.repeat(2048 - 'https://example.com/'.length);
			expect(exactLimitUrl.length).toBe(2048);
			expect(validateUrl(exactLimitUrl)).toBe(exactLimitUrl);
		});

		it('should block URLs exceeding the length limit', () => {
			// 2049 characters (over limit)
			const tooLongUrl = 'https://example.com/' + 'x'.repeat(2030);
			expect(tooLongUrl.length).toBeGreaterThan(2048);
			expect(validateUrl(tooLongUrl)).toBeNull();
		});

		it('should block extremely long URLs', () => {
			// 10KB URL
			const massiveUrl = 'https://example.com/' + 'x'.repeat(10000);
			expect(validateUrl(massiveUrl)).toBeNull();
			
			// 100KB URL
			const hugeUrl = 'https://example.com/' + 'x'.repeat(100000);
			expect(validateUrl(hugeUrl)).toBeNull();
		});

		it('should check length after trimming', () => {
			const urlWithWhitespace = '  https://example.com/' + 'x'.repeat(2030) + '  ';
			expect(urlWithWhitespace.length).toBeGreaterThan(2048);
			expect(validateUrl(urlWithWhitespace)).toBeNull();
		});
	});
});

describe('validateIframeUrl', () => {
	describe('with allowed domains', () => {
		it('should accept URLs from exact domain matches', () => {
			const domains = ['youtube.com', 'vimeo.com'];
			expect(validateIframeUrl('https://youtube.com/embed/abc', domains)).toBe('https://youtube.com/embed/abc');
			expect(validateIframeUrl('https://vimeo.com/123456', domains)).toBe('https://vimeo.com/123456');
		});

		it('should accept URLs with wildcard subdomain matches', () => {
			const domains = ['*.youtube.com', 'youtube.com'];
			expect(validateIframeUrl('https://www.youtube.com/embed/abc', domains)).toBe('https://www.youtube.com/embed/abc');
			expect(validateIframeUrl('https://m.youtube.com/embed/abc', domains)).toBe('https://m.youtube.com/embed/abc');
			expect(validateIframeUrl('https://youtube.com/embed/abc', domains)).toBe('https://youtube.com/embed/abc');
		});

		it('should block URLs from non-whitelisted domains', () => {
			const domains = ['youtube.com', 'vimeo.com'];
			expect(validateIframeUrl('https://evil.com/embed/malicious', domains)).toBeNull();
			expect(validateIframeUrl('https://attacker.com/phishing', domains)).toBeNull();
			expect(validateIframeUrl('https://example.com/video', domains)).toBeNull();
		});

		it('should work with DEFAULT_ALLOWED_DOMAINS', () => {
			expect(validateIframeUrl('https://youtube.com/embed/abc', DEFAULT_ALLOWED_DOMAINS)).toBeTruthy();
			expect(validateIframeUrl('https://www.youtube.com/embed/abc', DEFAULT_ALLOWED_DOMAINS)).toBeTruthy();
			expect(validateIframeUrl('https://youtu.be/abc', DEFAULT_ALLOWED_DOMAINS)).toBeTruthy();
			expect(validateIframeUrl('https://vimeo.com/123', DEFAULT_ALLOWED_DOMAINS)).toBeTruthy();
			expect(validateIframeUrl('https://player.vimeo.com/video/123', DEFAULT_ALLOWED_DOMAINS)).toBeTruthy();
		});
	});

	describe('without allowed domains', () => {
		it('should block all URLs when no domains are provided', () => {
			expect(validateIframeUrl('https://youtube.com/embed/abc', [])).toBeNull();
			expect(validateIframeUrl('https://example.com/video', [])).toBeNull();
		});

		it('should block when allowedDomains is undefined', () => {
			expect(validateIframeUrl('https://youtube.com/embed/abc', undefined as any)).toBeNull();
		});
	});

	describe('wildcard domain matching', () => {
		it('should match subdomains with wildcard', () => {
			const domains = ['*.example.com'];
			expect(validateIframeUrl('https://sub.example.com/video', domains)).toBeTruthy();
			expect(validateIframeUrl('https://deep.sub.example.com/video', domains)).toBeTruthy();
		});

		it('should match base domain with wildcard present', () => {
			const domains = ['*.example.com'];
			expect(validateIframeUrl('https://example.com/video', domains)).toBeTruthy();
		});

		it('should not match different domains with wildcard', () => {
			const domains = ['*.example.com'];
			expect(validateIframeUrl('https://notexample.com/video', domains)).toBeNull();
			expect(validateIframeUrl('https://example.org/video', domains)).toBeNull();
		});
	});
});
