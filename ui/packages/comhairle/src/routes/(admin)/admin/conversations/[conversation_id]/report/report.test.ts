import { describe, expect, it, vi } from 'vitest';
import type { ConversationAction } from '@crownshy/api-client/api';
import { load } from './+page';

function loadEvent(actions: ConversationAction[] | null) {
	const report = { id: 'report' };
	const api = {
		GetReportForConversation: vi.fn().mockResolvedValue(report),
		GenerateReportForConversation: vi.fn().mockResolvedValue(report)
	};
	const event = {
		depends: vi.fn(),
		parent: vi.fn().mockResolvedValue({
			conversation: { id: 'conversation' },
			api,
			workflowSteps: [],
			conversationActions:
				actions === null
					? null
					: {
							resourceType: 'conversation',
							resourceId: 'conversation',
							actions
						}
		})
	} as Parameters<typeof load>[0];
	return { event, api, report };
}

describe('Report read-only loading', () => {
	it('allows non-editors to read existing reports', async () => {
		const { event, api, report } = loadEvent(['conversation_read']);

		await expect(load(event)).resolves.toMatchObject({ report });
		expect(api.GetReportForConversation).toHaveBeenCalledWith({
			params: { conversation_id: 'conversation' },
			queries: { withTranslations: true }
		});
		expect(api.GenerateReportForConversation).not.toHaveBeenCalled();
	});

	it.each<ConversationAction>([
		'conversation_read',
		'conversation_export',
		'conversation_moderate',
		'conversation_translate'
	])('does not generate reports with only %s permission', async (action) => {
		const { event, api } = loadEvent([action]);
		const error = new Error('No report found');
		api.GetReportForConversation.mockRejectedValue(error);

		await expect(load(event)).rejects.toMatchObject({
			status: 403,
			body: { message: 'User lacks permission to update conversation' }
		});
		expect(api.GenerateReportForConversation).not.toHaveBeenCalled();
	});

	it('does not generate reports when permissions are unavailable', async () => {
		const { event, api } = loadEvent(null);
		const error = new Error('No report found');
		api.GetReportForConversation.mockRejectedValue(error);

		await expect(load(event)).rejects.toMatchObject({
			status: 403,
			body: { message: 'User lacks permission to update conversation' }
		});
		expect(api.GenerateReportForConversation).not.toHaveBeenCalled();
	});

	it('preserves report generation for editors', async () => {
		const { event, api, report } = loadEvent(['conversation_read', 'conversation_update']);
		api.GetReportForConversation.mockRejectedValue(new Error('No report found'));

		await expect(load(event)).resolves.toMatchObject({ report });
		expect(api.GenerateReportForConversation).toHaveBeenCalledWith(undefined, {
			params: { conversation_id: 'conversation' }
		});
	});
});
