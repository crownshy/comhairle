import type { LocalizedConversationDto } from '@crownshy/api-client/api';
import { apiClient } from '@crownshy/api-client/client';
import type { CreationKey } from './tool_meta';
import {
	basic_learn_config,
	basic_polis_config,
	basic_survey_config,
	basic_lived_experience_config,
	basic_elicitation_bot_config,
	basic_thinking_space_config,
	basic_prioritization_config,
	defaultStepCreationParams
} from './workflow_templates';

/** The `tool_setup` field type expected by the create-step endpoint. */
type CreateStepBody = Parameters<typeof apiClient.CreateConversationWorkflowStep>[0];

/** Only the id is read here; kept minimal so both the board and layout can pass their
 *  loaded conversation without a full {@link LocalizedConversationDto}. */
type ConversationRef = { id: string };

/**
 * Resolve the starter `tool_setup` config for a palette creation key (the value on
 * {@link import('./tool_meta').ToolMeta.creationKey}). Returns `undefined` for an
 * unknown key so callers can bail cleanly.
 *
 * @param creationKey - e.g. `'Polis'`, `'Learn'`, `'Elicitation Bot'`.
 * @param conversation - needed because the elicitation-bot config is conversation-scoped.
 */
export function toolSetupForCreationKey(creationKey: CreationKey, conversation: ConversationRef) {
	// `satisfies Record<CreationKey, ...>` makes this lookup exhaustive: adding a new
	// CreationKey without a config here is a compile error.
	const configByCreationKey = {
		Polis: basic_polis_config,
		Learn: basic_learn_config,
		Survey: basic_survey_config,
		'Lived Experience': basic_lived_experience_config,
		// basic_elicitation_bot_config only reads `conversation.id`; the cast avoids
		// requiring the full LocalizedConversationDto that callers don't have to hand.
		'Elicitation Bot': basic_elicitation_bot_config(conversation as LocalizedConversationDto),
		'Thinking Space': basic_thinking_space_config(),
		Prioritization: basic_prioritization_config
	} satisfies Record<CreationKey, unknown>;
	return configByCreationKey[creationKey];
}

/**
 * The `step_order` a newly appended step should take: one past the current maximum,
 * or 1 for an empty workflow. Pure so it can be unit-tested without the API.
 */
export function nextStepOrder(existingSteps: { stepOrder: number }[]): number {
	return existingSteps.length > 0 ? Math.max(...existingSteps.map((s) => s.stepOrder)) + 1 : 1;
}

/**
 * Create a workflow step from a palette creation key and append it to the workflow.
 * Single source of truth shared by the design board and the design layout's
 * add-step dialog (previously duplicated in both). Returns the created step (so the
 * caller can highlight it), or `undefined` if the creation key is unknown.
 */
export async function createWorkflowStep(params: {
	conversation: ConversationRef;
	workflowId: string;
	creationKey: CreationKey;
	existingSteps: { stepOrder: number }[];
}): Promise<Awaited<ReturnType<typeof apiClient.CreateConversationWorkflowStep>> | undefined> {
	const { conversation, workflowId, creationKey, existingSteps } = params;
	const tool_setup = toolSetupForCreationKey(creationKey, conversation);
	if (!tool_setup) return undefined;

	return apiClient.CreateConversationWorkflowStep(
		{
			name: defaultStepCreationParams[creationKey]?.name ?? `New ${creationKey} Step`,
			description:
				defaultStepCreationParams[creationKey]?.description ?? `A new ${creationKey} Step`,
			is_offline: false,
			activation_rule: 'manual',
			step_order: nextStepOrder(existingSteps),
			// The basic_*_config objects are structurally looser than the endpoint's
			// zod-inferred union (string vs literal `type`); this matches how the
			// previous inline add-step code passed them.
			tool_setup: tool_setup as CreateStepBody['tool_setup'],
			required: true
		},
		{ params: { conversation_id: conversation.id, workflow_id: workflowId } }
	);
}
