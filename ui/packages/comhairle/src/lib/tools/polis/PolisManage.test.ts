import { beforeEach, describe, expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import { permissions } from '$lib/permissions.svelte';
import PolisManage from './PolisManage.svelte';

vi.mock('$app/navigation', () => ({ invalidate: vi.fn() }));
vi.mock('$app/state', () => ({
	page: {
		url: new URL('http://localhost/admin/conversations/conversation/design/step/step/setup')
	}
}));
vi.mock('@crownshy/api-client/client', () => ({ apiClient: {} }));
vi.mock('$lib/permissions.svelte', () => ({ permissions: { can: vi.fn() } }));

const props = {
	conversationId: 'conversation',
	workflowId: 'workflow',
	workflowStepId: 'step',
	isLive: false,
	toolConfig: {
		topic: 'Transport',
		description: 'Buses and trains',
		required_votes: 10,
		show_remaining_statements: true,
		strict_moderation: true
	}
};

function renderControls(actions: string[]) {
	vi.mocked(permissions.can).mockImplementation(
		(resource, action, resourceId) =>
			resource === 'conversation' &&
			resourceId === props.conversationId &&
			actions.includes(action)
	);
	const { body } = render(PolisManage, { props });
	const controls =
		body.match(
			/<(?:input|textarea|button)\b[^>]*\bid="(?:topic|description|requiredVotes|showRemaining|strictModeration)"[^>]*>/g
		) ?? [];
	expect(controls).toHaveLength(5);
	return { body, controls };
}

describe('Polis setup permissions', () => {
	beforeEach(() => vi.clearAllMocks());

	it.each([
		['observer', ['conversation_read']],
		['moderator', ['conversation_read', 'conversation_moderate']],
		['exporter', ['conversation_read', 'conversation_export']],
		['missing permissions', []]
	])('disables all configuration fields for %s', (_role, actions) => {
		const { body, controls } = renderControls(actions);
		expect(controls.every((control) => /\sdisabled(?:[=\s>])/.test(control))).toBe(true);
		expect(body).toContain('Transport');
		expect(body).toContain('Buses and trains');
		expect(body).toContain('value="10"');
		expect(body).toContain('aria-checked="true"');
	});

	it('keeps all configuration fields interactive for editors', () => {
		const { controls } = renderControls(['conversation_read', 'conversation_update']);
		expect(controls.every((control) => !/\sdisabled(?:[=\s>])/.test(control))).toBe(true);
	});
});
