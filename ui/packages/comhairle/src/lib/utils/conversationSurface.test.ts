import { describe, expect, it } from 'vitest';
import type { ConversationAction } from '@crownshy/api-client/api';
import { conversationSurface, sectionSegment, surfaceRedirect } from './conversationSurface';

const holding =
	(...actions: ConversationAction[]) =>
	(action: ConversationAction) =>
		actions.includes(action);

describe('conversationSurface', () => {
	it('gives anyone who can edit the full surface', () => {
		expect(conversationSurface(holding('conversation_update'))).toBe('full');
		expect(conversationSurface(holding('conversation_update', 'conversation_moderate'))).toBe(
			'full'
		);
	});

	it('gives a moderator who cannot edit the moderation surface', () => {
		expect(conversationSurface(holding('conversation_read', 'conversation_moderate'))).toBe(
			'moderation'
		);
	});

	it('leaves other read-only roles on the full surface', () => {
		expect(conversationSurface(holding('conversation_read'))).toBe('full');
	});
});

describe('sectionSegment', () => {
	it('reads the section from a conversation admin path', () => {
		expect(sectionSegment('/admin/conversations/c1/configure/details', 'c1')).toBe('configure');
		expect(sectionSegment('/admin/conversations/c1/moderate/step/s1', 'c1')).toBe('moderate');
	});

	it('is null at the conversation root or outside it', () => {
		expect(sectionSegment('/admin/conversations/c1', 'c1')).toBeNull();
		expect(sectionSegment('/admin/conversations/c1/', 'c1')).toBeNull();
		expect(sectionSegment('/admin/conversations/c12/configure', 'c1')).toBeNull();
	});
});

describe('surfaceRedirect', () => {
	it('never moves someone on the full surface', () => {
		expect(surfaceRedirect('full', 'configure')).toBeNull();
	});

	it('sends a moderator anywhere else to Moderate', () => {
		expect(surfaceRedirect('moderation', 'configure')).toBe('moderate');
		expect(surfaceRedirect('moderation', 'launch')).toBe('moderate');
		expect(surfaceRedirect('moderation', null)).toBe('moderate');
	});

	it('lets a moderator stay on Moderate', () => {
		expect(surfaceRedirect('moderation', 'moderate')).toBeNull();
	});
});
