import type { ConversationStatus } from '$lib/utils/conversationStatus';

type NextStep = { label: string; section: 'configure' | 'monitor' | 'report' };

const NEXT_STEPS: Record<ConversationStatus, NextStep> = {
	draft: { label: 'Continue setting up', section: 'configure' },
	live: { label: 'Monitor responses', section: 'monitor' },
	completed: { label: 'Write the report', section: 'report' }
};

/** Where a conversation's next piece of work is, given its status. */
export function nextStep(conversationId: string, status: ConversationStatus) {
	const { label, section } = NEXT_STEPS[status];
	return { label, href: `/admin/conversations/${conversationId}/${section}` };
}
