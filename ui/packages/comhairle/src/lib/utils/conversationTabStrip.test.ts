import { describe, it, expect } from 'vitest';
import { conversationPrimaryStripSkeleton } from './conversationTabStrip';

const id = '29a4318b-4b91-4852-bb81-992c35d29b96';
const base = `/admin/conversations/${id}`;

describe('conversationPrimaryStripSkeleton', () => {
	it('returns a skeleton shape for sections that inject a primary strip', () => {
		expect(conversationPrimaryStripSkeleton(`${base}/configure`, id)).toEqual({
			leadingIcon: false,
			widths: [3.5, 4, 3, 2.75]
		});
		expect(conversationPrimaryStripSkeleton(`${base}/design`, id)?.leadingIcon).toBe(true);
		expect(conversationPrimaryStripSkeleton(`${base}/events`, id)?.leadingIcon).toBe(true);
		expect(conversationPrimaryStripSkeleton(`${base}/invites`, id)).not.toBeNull();
	});

	it('resolves nested routes to their section skeleton', () => {
		expect(conversationPrimaryStripSkeleton(`${base}/design/step/abc`, id)?.leadingIcon).toBe(
			true
		);
		expect(
			conversationPrimaryStripSkeleton(`${base}/events/some-event-id`, id)?.leadingIcon
		).toBe(true);
	});

	it('ignores a trailing slash', () => {
		expect(conversationPrimaryStripSkeleton(`${base}/configure/`, id)).not.toBeNull();
	});

	it('returns null for sections without a primary strip', () => {
		expect(conversationPrimaryStripSkeleton(`${base}/knowledge-base`, id)).toBeNull();
		expect(conversationPrimaryStripSkeleton(`${base}/monitor`, id)).toBeNull();
		expect(conversationPrimaryStripSkeleton(`${base}/report`, id)).toBeNull();
		expect(conversationPrimaryStripSkeleton(base, id)).toBeNull();
	});

	it('returns null for unrelated paths or a mismatched conversation id', () => {
		expect(conversationPrimaryStripSkeleton('/admin/dashboard', id)).toBeNull();
		expect(
			conversationPrimaryStripSkeleton(`/admin/conversations/other-id/configure`, id)
		).toBeNull();
	});

	it('does not match a section that only shares a prefix', () => {
		expect(conversationPrimaryStripSkeleton(`${base}/designs`, id)).toBeNull();
		expect(conversationPrimaryStripSkeleton(`${base}/configure-extra`, id)).toBeNull();
	});
});
