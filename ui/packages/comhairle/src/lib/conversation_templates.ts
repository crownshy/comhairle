import {
	basic_learn_config,
	basic_survey_config,
	basic_polis_config,
	basic_prioritization_config
} from './workflow_templates';

/**
 * A single step as instantiated on the backend when a conversation is created
 * from a template. Mirrors the CreateWorkflowStep body shape.
 */
export type CreationStep = {
	name: string;
	description: string;
	is_offline: boolean;
	activation_rule: string;
	step_order: number;
	tool_setup: unknown;
	required: boolean;
};

/** A step as shown on the template card's accordion. Display-only. */
export type DisplayStep = {
	label: string;
	description: string;
};

export type TemplateBadge = {
	label: string;
	class: string;
};

/**
 * A placeholder Event created alongside the conversation. Used for template
 * steps that are conceptually an event (e.g. a video conference) rather than a
 * workflow tool. Times are computed at creation from a base "now".
 */
export type CreationEvent = {
	name: string;
	description: string;
	signup_mode: 'open' | 'invite';
	/** Event length in minutes, from a base start time of "now". */
	durationMinutes: number;
};

export type ConversationTemplate = {
	/** Stable key used by createConversation(). */
	key: string;
	name: string;
	/** Short subtitle shown in the selectable list on the left. */
	description: string;
	badges: TemplateBadge[];
	/**
	 * Steps shown in the card accordion. May include steps whose backing tool
	 * does not yet exist (e.g. "Online video conference") — these are display
	 * only and are NOT instantiated.
	 */
	displaySteps: DisplayStep[];
	/**
	 * Real steps instantiated on "Get started". A subset of displaySteps: only
	 * steps with a backing tool appear here. Empty array => empty workflow.
	 */
	creationSteps: CreationStep[];
	/**
	 * Placeholder events created alongside the conversation, for display steps
	 * that are events rather than workflow tools (e.g. video conference).
	 */
	creationEvents?: CreationEvent[];
	/** Whether the card is selectable. False => "coming soon" / disabled. */
	available: boolean;
};

const BADGE = {
	online: { label: 'Online', class: 'bg-cyan-200/20' },
	broadAudience: { label: 'Broad audience', class: 'bg-amber-400/20' },
	limitedCapacity: { label: 'Limited team capacity', class: 'bg-pink-400/20' },
	inPerson: { label: 'In person', class: 'bg-pink-400/20' },
	oneDay: { label: '1 day', class: 'bg-cyan-200/20' },
	cycleDevelopment: { label: 'Delivery Cycle: Development', class: 'bg-amber-400/20' },
	cycleAppraisal: { label: 'Delivery Cycle: Appraisal', class: 'bg-accent' }
} as const;

const step = {
	topicOnboarding: {
		label: 'Topic onboarding',
		description:
			'Present participants with information to help them learn about the topic at hand'
	},
	survey: {
		label: 'Survey',
		description: 'Ask participants a series of questions'
	},
	wikiPoll: {
		label: 'Wiki-poll (Pol.is)',
		description:
			'Ask for peoples views. Allow them to vote on others views. Understand the landscape of opinions'
	},
	videoConference: {
		label: 'Online video conference',
		description: 'Facilitate live video meetings for real-time collaboration and discussion.'
	},
	prioritisation: {
		label: 'Proposal prioritisation',
		description:
			'Collect a set of proposals and have participants score each one against a shared set of questions.'
	},
	thinkingSpace: {
		label: 'Thinking Space',
		description:
			"Understand participant's background and questions they might have to the topic"
	},
	workshop: {
		label: 'Workshop',
		description: 'Join in-person workshop with other participants'
	},
	// Distinct from `wikiPoll` above: the Citizen workshop card labels and
	// describes its Polis step differently ("Wiki Poll (Polis)" vs "Wiki-poll
	// (Pol.is)"), so it needs its own display entry to stay 1:1 with the design.
	wikiPollWorkshop: {
		label: 'Wiki Poll (Polis)',
		description: 'Reflecting where they are with other people'
	}
} as const;

// Reusable creation-step builders (order is assigned per template below).
const learnStep = (order: number): CreationStep => ({
	name: 'Learn about the topic',
	description:
		'Before we can hear what you think, we want to tell you a bit more about the topic at hand',
	is_offline: false,
	activation_rule: 'manual',
	step_order: order,
	tool_setup: basic_learn_config,
	required: true
});

const surveyStep = (order: number): CreationStep => ({
	name: 'Take a short survey',
	description: 'Take a short survey about your views',
	is_offline: false,
	activation_rule: 'manual',
	step_order: order,
	tool_setup: basic_survey_config,
	required: true
});

const polisStep = (order: number): CreationStep => ({
	name: 'Tell us what you think',
	description: 'Tell us what you think below and vote on what others think',
	is_offline: false,
	activation_rule: 'manual',
	step_order: order,
	tool_setup: basic_polis_config,
	required: true
});

const prioritisationStep = (order: number): CreationStep => ({
	name: 'Rate the proposals',
	description: 'Read each proposal and rate it against the questions provided.',
	is_offline: false,
	activation_rule: 'manual',
	step_order: order,
	tool_setup: basic_prioritization_config,
	required: true
});

export const conversationTemplates: ConversationTemplate[] = [
	{
		key: 'simple_survey',
		name: 'Informed-participants survey',
		description: 'Interview participants with planned questions',
		badges: [BADGE.online, BADGE.broadAudience],
		displaySteps: [step.topicOnboarding, step.survey],
		creationSteps: [learnStep(1), surveyStep(2)],
		available: true
	},
	{
		key: 'understand_opinion_groups',
		name: 'Understand opinion groups',
		description: 'Get a sense of what people think on a given topic',
		badges: [BADGE.online, BADGE.limitedCapacity],
		displaySteps: [step.topicOnboarding, step.wikiPoll, step.survey],
		creationSteps: [learnStep(1), polisStep(2), surveyStep(3)],
		available: true
	},
	{
		key: 'stakeholder_engagement',
		name: 'Closed-group engagement',
		description: 'Ask question to stakeholders on policy decisions and choices',
		badges: [BADGE.cycleDevelopment],
		// "Online video conference" has no backing workflow tool, so the two
		// backed steps (learn + prioritisation) are created as normal and the
		// video step becomes an empty placeholder event the admin fills in.
		displaySteps: [step.topicOnboarding, step.videoConference, step.prioritisation],
		creationSteps: [learnStep(1), prioritisationStep(2)],
		creationEvents: [
			{
				name: 'Online video conference',
				description: '',
				signup_mode: 'open',
				durationMinutes: 60
			}
		],
		available: true
	},
	{
		key: 'compare_proposals',
		name: 'Compare proposals',
		description: 'Present multiple proposals and collect comments',
		badges: [BADGE.cycleAppraisal],
		displaySteps: [step.topicOnboarding, step.prioritisation],
		creationSteps: [learnStep(1), prioritisationStep(2)],
		available: true
	},
	{
		key: 'citizen_workshop',
		name: 'Citizen workshop',
		description: 'Engage with citizens in person for quality communication and conversations',
		badges: [BADGE.inPerson, BADGE.oneDay, BADGE.cycleDevelopment],
		// "Thinking Space" and "Workshop" have no backing tool, so they are display
		// only. creationSteps is deliberately left as learn + prioritisation (the
		// created workflow is unchanged); revisit if these should instantiate real
		// steps.
		displaySteps: [
			step.topicOnboarding,
			step.thinkingSpace,
			step.workshop,
			step.wikiPollWorkshop
		],
		creationSteps: [learnStep(1), prioritisationStep(2)],
		available: true
	}
];

export function getTemplate(key: string): ConversationTemplate | undefined {
	return conversationTemplates.find((t) => t.key === key);
}
