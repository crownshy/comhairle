import { describe, expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import type { ConversationAction } from '@crownshy/api-client/api';
import ConversationTabs from '$lib/components/ConversationTabs.svelte';
import { permissions } from '$lib/permissions.svelte';
import { load } from './+page.server';

vi.mock('$app/state', () => ({
	page: { url: new URL('http://localhost/admin/conversations/conversation/monitor') },
	navigating: { to: null }
}));
vi.mock('$lib/permissions.svelte', () => ({ permissions: { can: vi.fn() } }));

const conversationId = 'conversation';

function routeEvent(actions: ConversationAction[]) {
	const api = {
		GetUserActions: vi.fn().mockResolvedValue({
			resourceType: 'conversation',
			resourceId: conversationId,
			actions
		})
	};
	const event = {
		params: { conversation_id: conversationId },
		parent: vi.fn().mockResolvedValue({}),
		locals: { api }
	} as Parameters<typeof load>[0];
	return { event, api };
}

describe('Notify Admin access', () => {
	it('renders Notify as disabled without Admin while leaving Monitor accessible', () => {
		vi.mocked(permissions.can).mockReturnValue(false);
		const { body } = render(ConversationTabs, {
			props: { conversationId, conversationIsLive: true }
		});

		expect(body).toContain('Notify');
		expect(body).toContain('Admin permission required');
		expect(body).not.toContain(`href="/admin/conversations/${conversationId}/notifications"`);
		expect(body).toContain(`href="/admin/conversations/${conversationId}/monitor"`);
		expect(permissions.can).toHaveBeenCalledWith(
			'conversation',
			'conversation_admin',
			conversationId
		);
	});

	it('renders the Notify link for Admins', () => {
		vi.mocked(permissions.can).mockReturnValue(true);
		const { body } = render(ConversationTabs, {
			props: { conversationId, conversationIsLive: true }
		});

		expect(body).toContain(`href="/admin/conversations/${conversationId}/notifications"`);
	});

	it.each<ConversationAction>([
		'conversation_read',
		'conversation_update',
		'conversation_export',
		'conversation_moderate',
		'conversation_translate'
	])('redirects direct visits with only %s permission', async (action) => {
		const { event } = routeEvent([action]);

		await expect(load(event)).rejects.toMatchObject({
			status: 303,
			location: `/admin/conversations/${conversationId}/monitor`
		});
	});

	it('redirects direct visits without permissions', async () => {
		const { event } = routeEvent([]);

		await expect(load(event)).rejects.toMatchObject({
			status: 303,
			location: `/admin/conversations/${conversationId}/monitor`
		});
	});

	it('fails closed when permissions are unavailable', async () => {
		const { event, api } = routeEvent([]);
		api.GetUserActions.mockRejectedValue(new Error('Permissions unavailable'));

		await expect(load(event)).rejects.toMatchObject({
			status: 500,
			body: { message: 'Unable to load conversation permissions.' }
		});
	});

	it.each([
		{ resourceType: 'organization', resourceId: conversationId },
		{ resourceType: 'conversation', resourceId: 'another-conversation' }
	])('fails closed for mismatched permissions: %j', async (resource) => {
		const { event, api } = routeEvent(['conversation_admin']);
		api.GetUserActions.mockResolvedValue({
			...resource,
			actions: ['conversation_admin']
		});

		await expect(load(event)).rejects.toMatchObject({
			status: 500,
			body: { message: 'Unable to load conversation permissions.' }
		});
	});

	it('allows direct visits with Admin permission', async () => {
		const { event, api } = routeEvent(['conversation_read', 'conversation_admin']);

		await expect(load(event)).resolves.toEqual({});
		expect(event.parent).toHaveBeenCalledOnce();
		expect(api.GetUserActions).toHaveBeenCalledWith({
			params: { resource_type: 'conversation', resource_id: conversationId }
		});
	});
});
