import type { LocalizedConversationDto } from './api/api';

export const defaultStepCreationParams: { [key: string]: { name: string; description: string } } = {
	'Elicitation Bot': {
		name: 'What do you think?',
		description:
			"Explore your opinions with the elicitation bot. The elicitation bot is designed to ask questions that help you explore and shape your views and opinions on a given topic.\nAs you answer the bot's questions it will extract claims from your opinions, which you can either choose to approve, edit or remove."
	},
	Prioritization: {
		name: 'Rate the proposals',
		description: 'Read each proposal and rate it against the questions provided.'
	},
	'Thinking Space': {
		name: 'Thinking Space',
		description:
			"You'll be asked a few questions, and after each one you can pick follow-up questions to dig into. It's a chance to go deeper on what you really think. At the end you'll be able to review and edit your responses before submitting."
	}
};

export const basic_learn_config = {
	type: 'learn',
	pages: [
		[
			{
				content: '# Page 1 \n\n',
				lang: 'en',
				type: 'markdown'
			},

			{
				content: '# Leathanach 1 \n\n',
				lang: 'gd',
				type: 'markdown'
			}
		],
		[
			{
				content: '# Page 2 \n\n',
				lang: 'en',
				type: 'markdown'
			},

			{
				content: '# Leathanach 2 \n\n',
				lang: 'gd',
				type: 'markdown'
			}
		]
	]
};

export const basic_polis_config = {
	type: 'polis',
	topic: 'new polis poll',
	show_remaining_statements: true
};

export const basic_survey_config = {
	type: 'heyform'
};

export const basic_elicitation_bot_config = (conversation: LocalizedConversationDto) => ({
	type: 'elicitationbot',
	topic: 'comhairle platform',
	conversation_id: conversation.id
});

export const basic_thinking_space_config = () => ({
	type: 'thinkingspace',
	topic: 'What do you think?',
	root_questions: [],
	follow_up_rounds_count: 2
});

export const basic_lived_experience_config = {
	type: 'stories',
	max_time: 10,
	to_see: 3
};

export const basic_prioritization_config = {
	type: 'prioritization',
	randomize_order: false,
	questions: [
		{
			id: crypto.randomUUID(),
			text: 'How strongly do you support this proposal?',
			type: {
				likert_scale: {
					categories: [
						{ label: 'Strongly disagree', value: 1 },
						{ label: 'Disagree', value: 2 },
						{ label: 'Neutral', value: 3 },
						{ label: 'Agree', value: 4 },
						{ label: 'Strongly agree', value: 5 }
					]
				}
			}
		}
	]
};
export const workflow_templates = {
	learn_polis: [
		{
			name: 'Learn about the topic',
			description:
				'Before we can hear what you think, we want to tell you a bit more about the topic at hand',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 1,
			tool_setup: basic_learn_config,
			required: true
		},
		{
			name: 'Tell us what you think',
			description: 'Tell us what you think below and vote on what others think',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 2,
			tool_setup: basic_polis_config,
			required: true
		}
	],
	learn_survey: [
		{
			name: 'Learn about the topic',
			description:
				'Before we can hear what you think, we want to tell you a bit more about the topic at hand',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 1,
			tool_setup: basic_learn_config,
			required: true
		},

		{
			name: 'Take a short survey',
			description: 'Take a short survey about your views',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 2,
			tool_setup: basic_survey_config,
			required: true
		}
	],
	learn_survey_polis: [
		{
			name: 'Learn about the topic',
			description:
				'Before we can hear what you think, we want to tell you a bit more about the topic at hand',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 1,
			tool_setup: basic_learn_config,
			required: true
		},

		{
			name: 'Take a short survey',
			description: 'Take a short survey about your views',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 2,
			tool_setup: basic_survey_config,
			required: true
		},

		{
			name: 'Tell us what you think',
			description: 'Tell us what you think bellow and vote on what others thing',
			is_offline: false,
			activation_rule: 'manual',
			step_order: 3,
			tool_setup: basic_polis_config,
			required: true
		}
	],
	empty: []
};
