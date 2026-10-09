import { describe, expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import { permissions } from '$lib/permissions.svelte';
import WorkflowStepStrip from './WorkflowStepStrip.svelte';

vi.mock('$app/state', () => ({
	page: { url: new URL('http://localhost/admin/conversations/conversation/design') }
}));
vi.mock('$lib/permissions.svelte', () => ({ permissions: { can: vi.fn() } }));

describe('Workflow navigation permissions', () => {
	it.each([
		['observer', ['conversation_read'], true],
		['moderator', ['conversation_read', 'conversation_moderate'], true],
		['exporter', ['conversation_read', 'conversation_export'], true],
		['missing permissions', [], true],
		['editor', ['conversation_read', 'conversation_update'], false]
	] satisfies [string, string[], boolean][])(
		'sets Add step disabled correctly for %s',
		(_role, actions, disabled) => {
			vi.mocked(permissions.can).mockImplementation(
				(resource, action, resourceId) =>
					resource === 'conversation' &&
					resourceId === 'conversation' &&
					actions.includes(action)
			);
			const { body } = render(WorkflowStepStrip, {
				props: { conversationId: 'conversation', steps: [], onAddStep: vi.fn() }
			});
			const button = body.match(/<button\b[^>]*>/)?.[0];
			expect(button).toBeDefined();
			expect(/\sdisabled(?:[=\s>])/.test(button ?? '')).toBe(disabled);
			expect(body).toContain('Add step');
			expect(body).toContain('href="/admin/conversations/conversation/design"');
			if (disabled) expect(button).toContain('Edit permission required');
		}
	);
});
