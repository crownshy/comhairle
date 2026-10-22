import { describe, expect, it } from 'vitest';
import { isPreviewPasswordRequired, previewGrantCookies, previewUnlockUrl } from './previewAccess';

describe('previewGrantCookies', () => {
	it('keeps only preview grant cookies, as name=value pairs', () => {
		const cookies = {
			getAll: () => [
				{ name: 'auth-token', value: 'session' },
				{ name: 'preview-access-abc', value: 'grant-a' },
				{ name: 'COMHAIRLE_LOCALE', value: 'en' },
				{ name: 'preview-access-def', value: 'grant-b' }
			]
		};

		expect(previewGrantCookies(cookies)).toEqual([
			'preview-access-abc=grant-a',
			'preview-access-def=grant-b'
		]);
	});
});

describe('isPreviewPasswordRequired', () => {
	it('matches the API error for a locked preview', () => {
		const error = { response: { status: 401, data: { err: 'Preview password required' } } };
		expect(isPreviewPasswordRequired(error)).toBe(true);
	});

	it('ignores other unauthorised errors', () => {
		const error = { response: { status: 401, data: { err: 'No user logged in' } } };
		expect(isPreviewPasswordRequired(error)).toBe(false);
	});

	it('ignores errors without a response', () => {
		expect(isPreviewPasswordRequired(new Error('network'))).toBe(false);
		expect(isPreviewPasswordRequired(null)).toBe(false);
	});
});

describe('previewUnlockUrl', () => {
	it('sends the visitor back to where they were going', () => {
		expect(previewUnlockUrl('my-slug', '/conversations/my-slug/preview?x=1')).toBe(
			'/conversations/my-slug/preview-unlock?backTo=%2Fconversations%2Fmy-slug%2Fpreview%3Fx%3D1'
		);
	});
});
