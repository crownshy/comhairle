import { describe, expect, it } from 'vitest';
import { DEFAULT_CONVERSATION_IMAGE, conversationImageUrl } from './conversationImage';

describe('conversationImageUrl', () => {
	it('keeps an image the admin actually uploaded', () => {
		expect(conversationImageUrl('https://media.comhairle.scot/images/banner.png')).toBe(
			'https://media.comhairle.scot/images/banner.png'
		);
	});

	it('falls back when the field is empty or missing', () => {
		expect(conversationImageUrl('')).toBe(DEFAULT_CONVERSATION_IMAGE);
		expect(conversationImageUrl('   ')).toBe(DEFAULT_CONVERSATION_IMAGE);
		expect(conversationImageUrl(null)).toBe(DEFAULT_CONVERSATION_IMAGE);
		expect(conversationImageUrl(undefined)).toBe(DEFAULT_CONVERSATION_IMAGE);
	});

	it('falls back when the API substituted its own placeholder', () => {
		expect(
			conversationImageUrl(
				'https://comhairle-media.s3.amazonaws.com/images/comhairle-conversation-placeholder.png'
			)
		).toBe(DEFAULT_CONVERSATION_IMAGE);
		expect(
			conversationImageUrl(
				'https://comhairle-media-test.storage.com/images/comhairle-conversation-placeholder.png'
			)
		).toBe(DEFAULT_CONVERSATION_IMAGE);
	});
});
