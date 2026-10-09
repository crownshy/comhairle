import type { LocalizedConversationDto, WorkflowStepDto } from '@crownshy/api-client/api';
import { apiClient } from '@crownshy/api-client/client';
import type { CreationKey } from '$lib/tool_meta';
import {
	basic_learn_config,
	basic_polis_config,
	basic_survey_config,
	basic_lived_experience_config,
	basic_elicitation_bot_config,
	basic_thinking_space_config,
	basic_prioritization_config,
	defaultStepCreationParams
} from '$lib/workflow_templates';
import { tryCatchAsync, type ErrorType, type Result } from '$lib/utils/errorHandling';

/** The `tool_setup` field type expected by the create-step endpoint. */
type CreateStepBody = Parameters<typeof apiClient.CreateConversationWorkflowStep>[0];

/**
 * Resolve the starter `tool_setup` config for a palette creation key (the value on
 * {@link import('./tool_meta').ToolMeta.creationKey}). Returns `undefined` for an
 * unknown key so callers can bail cleanly.
 *
 * @param creationKey - e.g. `'Polis'`, `'Learn'`, `'Elicitation Bot'`.
 * @param conversation - needed because the elicitation-bot config is conversation-scoped.
 */
export function toolSetupForCreationKey(
	creationKey: CreationKey,
	conversationId?: LocalizedConversationDto['id']
) {
	switch (creationKey) {
		case 'Polis':
			return basic_polis_config;
		case 'Learn':
			return basic_learn_config;
		case 'Survey':
			return basic_survey_config;
		case 'Lived Experience':
			return basic_lived_experience_config;
		case 'Elicitation Bot':
			if (!conversationId) {
				return;
			}
			return basic_elicitation_bot_config(conversationId);
		case 'Thinking Space':
			return basic_thinking_space_config();
		case 'Prioritization':
			return basic_prioritization_config;
		default: {
			// Unreachable: `creationKey` is exhausted above. Assigning it to `never` makes a
			// newly added CreationKey without a case here a compile error.
			const _exhaustive: never = creationKey;
			return _exhaustive;
		}
	}
}

type CreateWorkflowStepParameters = {
	creationKey: CreationKey;
	conversationId: LocalizedConversationDto['id'];
	workflowId: string;
	// Highest step order out of all workflow steps
	highestStepOrder: number;
};
/**
 * Create a workflow step from a palette creation key and append it to the workflow.
 * Single source of truth shared by the design board and the design layout's
 * add-step dialog (previously duplicated in both). Returns the created step (so the
 * caller can highlight it), or `undefined` if the creation key is unknown.
 */
export const createWorkflowStep = async (
	params: CreateWorkflowStepParameters
): Promise<Result<'ok', WorkflowStepDto, ErrorType>> =>
	tryCatchAsync<WorkflowStepDto, ErrorType>(async (ok, err) => {
		const tool_setup = toolSetupForCreationKey(params.creationKey, params.conversationId);
		if (!tool_setup) {
			throw err('Could not get tool config');
		}

		const toolDefaults = defaultStepCreationParams(params.creationKey);

		const createWorkflowStep = await tryCatchAsync(() =>
			apiClient.CreateConversationWorkflowStep(
				{
					name: toolDefaults.name,
					description: toolDefaults.description,
					is_offline: false,
					activation_rule: 'manual',
					step_order: params.highestStepOrder + 1,
					// The basic_*_config objects are structurally looser than the endpoint's
					// zod-inferred union (string vs literal `type`); this matches how the
					// previous inline add-step code passed them.
					tool_setup: tool_setup as CreateStepBody['tool_setup'],
					required: true
				},
				{
					params: {
						conversation_id: params.conversationId,
						workflow_id: params.workflowId
					}
				}
			)
		);

		if (createWorkflowStep.err !== null) {
			throw err(createWorkflowStep.err);
		}

		return ok(createWorkflowStep.ok);
	});
