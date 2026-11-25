import { apiClient } from '@crownshy/api-client/client';
import type { InstancedToolConfig } from '$lib/tools/types';

// The rest of the config is display flags that only live in tool_config.
const POLIS_OWNED_FIELDS = new Set([
	'topic',
	'description',
	'is_active',
	'strict_moderation',
	'notify_moderators'
]);

export type PolisStepTarget = {
	conversationId: string;
	workflowId: string;
	workflowStepId: string;
	isLive: boolean;
	toolConfig: InstancedToolConfig<'polis'>;
};

/**
 * Polis-owned fields are written to Polis first, then mirrored into tool_config so the
 * form pre-fills next time. That order means a Polis failure never leaves a value in
 * tool_config that Polis didn't accept.
 */
export async function savePolisSetting(target: PolisStepTarget, field: string, value: unknown) {
	if (POLIS_OWNED_FIELDS.has(field)) {
		await apiClient.PolisUpdateConfig({
			workflow_step_id: target.workflowStepId,
			[field]: value
		});
	}
	await apiClient.UpdateConversationWorkflowStep(
		{
			[target.isLive ? 'tool_config' : 'preview_tool_config']: {
				...target.toolConfig,
				[field]: value
			}
		},
		{
			params: {
				conversation_id: target.conversationId,
				workflow_id: target.workflowId,
				workflow_step_id: target.workflowStepId
			}
		}
	);
}
