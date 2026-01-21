import type { Conversation } from './api/api';

export const basic_learn_config = {
	type: 'learn',
	pages: [
		[
			{
				content: '#Page 1 \n\n',
				lang: 'en',
				type: 'markdown'
			},

			{
				content: '#Leathanach 1 \n\n',
				lang: 'gd',
				type: 'markdown'
			}
		],
		[
			{
				content: '#Page 2 \n\n',
				lang: 'en',
				type: 'markdown'
			},

			{
				content: '#Leathanach 2 \n\n',
				lang: 'gd',
				type: 'markdown'
			}
		]
	]
};

export const basic_polis_config = {
	type: 'polis',
	topic: 'new polis poll'
};

export const basic_survey_config = {
	type: 'heyform'
};

export const basic_elicitation_bot_config = (conversation: Conversation) => ({
	type: 'elicitationbot',
	topic: 'comhairle platform',
	conversation_id: conversation.id
});

export const basic_lived_experience_config = {
	type: 'stories',
	max_time: 10,
	to_see: 3
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
			description: 'Tell us what you think bellow and vote on what others thing',
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
