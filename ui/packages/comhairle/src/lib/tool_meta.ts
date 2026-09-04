import {
	MessagesSquare,
	Video,
	ListChecks,
	ListOrdered,
	BookOpen,
	Bot,
	type Icon
} from 'lucide-svelte';

/**
 * The discriminant used on a step's `toolConfig.type` / `previewToolConfig.type`.
 * Mirrors the ToolConfig union in the generated api-client.
 */
export type ToolType =
	| 'polis'
	| 'learn'
	| 'heyform'
	| 'stories'
	| 'elicitationbot'
	| 'thinkingspace'
	| 'prioritization';

/**
 * PascalCase key that seeds a new step's starter config and default name.
 * Distinct from both {@link ToolType} (the machine discriminant, e.g. `heyform`) and
 * `displayName` (the verbose Figma label, e.g. `Survey`). Consumed by the switches in
 * {@link import('./createWorkflowStep').toolSetupForCreationKey} and
 * `defaultStepCreationParams`; typing it keeps those lookups exhaustive.
 */
export type CreationKey =
	| 'Learn'
	| 'Thinking Space'
	| 'Polis'
	| 'Survey'
	| 'Prioritization'
	| 'Elicitation Bot'
	| 'Lived Experience';

export type ToolMeta = {
	/** ToolConfig discriminant. */
	type: ToolType;
	/** Admin-facing label (Figma display name). Distinct from the internal creation key. */
	displayName: string;
	/** One-line tooltip / palette description. */
	tagline: string;
	/** Icon shown on cards and the palette. */
	icon: typeof Icon;
	/** Route slug under /admin/info/tools/<infoSlug>. */
	infoSlug: string;
	/** Key passed to the add-step switch (see {@link import('./createWorkflowStep').createWorkflowStep}). */
	creationKey: CreationKey;
	/**
	 * Typical duration in minutes for the tool as a whole. Only a fallback now: a step's
	 * quoted time comes from its own config where the config says anything about length
	 * (see `stepDuration.ts`), and this stands in for the tools whose config does not.
	 */
	estimatedMinutes: number;
	/**
	 * Fuller paragraph shown in the AddStepDialog detail panel (below the display name).
	 * Longer than {@link tagline}.
	 */
	description: string;
	/** "BEST FOR" bullets in the AddStepDialog detail panel. */
	bestFor: string[];
	/** "FEATURES" bullets in the AddStepDialog detail panel. */
	features: string[];
	/** "WHAT YOU'D GET" bullets in the AddStepDialog detail panel. */
	whatYoudGet: string[];
};

/**
 * Single source of truth for how each engagement tool is presented in the admin UI.
 * Keyed by the ToolConfig discriminant.
 *
 * NOTE: the `description` / `bestFor` / `features` / `whatYoudGet` copy is authored
 * detail-panel content. `learn`, `thinkingspace`, `polis` and `heyform` use the
 * finalised Figma copy; the others are still PLACEHOLDER copy derived from each
 * tool's tagline and need product/design sign-off before they ship.
 */
export const TOOL_META: Record<ToolType, ToolMeta> = {
	learn: {
		type: 'learn',
		displayName: 'Topic onboarding',
		tagline:
			'Present participants with information to help them learn about the topic at hand.',
		icon: BookOpen,
		infoSlug: 'learn',
		creationKey: 'Learn',
		estimatedMinutes: 10,
		description:
			'Help participants build understanding before or during engagement. Use the Topic Onboarding step to share text, images, videos, audio, and other rich media that provide context, guidance, or key information.',
		bestFor: ['Self-paced learning modules', 'Pre-reading before a discussion'],
		features: [
			'Rich media content support',
			'Optional Learning Assistant to support participants as they learn'
		],
		whatYoudGet: ['Completion rate', 'Time-on-task data']
	},
	thinkingspace: {
		type: 'thinkingspace',
		displayName: 'Individual view exploration',
		tagline:
			'Help participants explore their views by asking them non-leading coaching questions that broaden and deepen their views.',
		icon: Bot,
		infoSlug: 'thinking_space',
		creationKey: 'Thinking Space',
		estimatedMinutes: 12,
		description:
			'Help participants explore their views by asking them non-leading coaching questions that broaden and deepen their views.',
		bestFor: ['Reflective individual exercises', 'Preparation before group dialogue'],
		features: ['AI-guided coaching questions', 'Adaptive follow-up prompts'],
		whatYoudGet: [
			'Participant reflection responses (depend on consent)',
			'Participant reflection summaries (depend on consent)'
		]
	},
	polis: {
		type: 'polis',
		displayName: 'Participant-led poll',
		tagline:
			"Show participants others' views and vote 'Agree' 'Disagree' or 'Pass' if unable to decide. Participants can also make their own views for others to vote on.",
		icon: MessagesSquare,
		infoSlug: 'polis',
		creationKey: 'Polis',
		estimatedMinutes: 12,
		description:
			"Show participants others' views and vote 'Agree', 'Disagree' or 'Pass'. Participants can also submit their own views.",
		bestFor: [
			'Surfacing diverse perspectives',
			'Encourage broad participation in early stage discussion'
		],
		features: ['Real-time opinion group mapping', 'Participant-submitted statements'],
		whatYoudGet: ['Voting breakdowns by statement', 'Consensus and divergence insights']
	},
	heyform: {
		type: 'heyform',
		displayName: 'Survey',
		tagline: 'Ask participants a series of pre-planned questions.',
		icon: ListChecks,
		infoSlug: 'heyform',
		creationKey: 'Survey',
		estimatedMinutes: 9,
		description: 'Ask participants a series of pre-planned questions.',
		bestFor: ['Structured data collection and analysis', 'Pre- and post-engagement feedback'],
		features: ['Multiple question types', 'Conditional logic'],
		whatYoudGet: ['Aggregated responses', 'Individual response breakdown']
	},
	prioritization: {
		type: 'prioritization',
		displayName: 'Proposal prioritisation',
		tagline:
			'Collect a set of proposals and have participants score each one against a shared set of questions.',
		icon: ListOrdered,
		infoSlug: 'prioritization',
		creationKey: 'Prioritization',
		estimatedMinutes: 10,
		// PLACEHOLDER copy (needs product/design review).
		description:
			'Collect a set of proposals and have participants score each one against a shared set of questions, producing a ranked, comparable view of what matters most.',
		bestFor: ['Ranking ideas or options', 'Allocating limited resources'],
		features: ['Score proposals against shared criteria', 'Participant-submitted proposals'],
		whatYoudGet: ['Ranked proposals', 'Score breakdown per criterion']
	},
	elicitationbot: {
		type: 'elicitationbot',
		displayName: 'Elicitation Bot',
		tagline:
			'Help participants refine and capture their views through an AI bot mediated interaction.',
		icon: Bot,
		infoSlug: 'elicitation_bot',
		creationKey: 'Elicitation Bot',
		estimatedMinutes: 10,
		// PLACEHOLDER copy (needs product/design review).
		description:
			'Help participants refine and capture their views through an AI-mediated conversation. The bot asks questions, then extracts claims participants can approve, edit, or remove.',
		bestFor: [
			'Capturing nuanced individual views',
			'Turning free-form opinions into structured claims'
		],
		features: ['AI-guided questioning', 'Participant-approved claim extraction'],
		whatYoudGet: ['Structured claims per participant', 'Themes across contributions']
	},
	stories: {
		type: 'stories',
		displayName: 'Lived Experience',
		tagline: 'Let users record short videos of their lived experience.',
		icon: Video,
		infoSlug: 'lived_experience',
		creationKey: 'Lived Experience',
		estimatedMinutes: 8,
		// PLACEHOLDER copy (needs product/design review).
		description:
			'Let participants record short videos sharing their lived experience, capturing context and emotion that text alone can miss.',
		bestFor: ['Gathering personal testimony', 'Human-centred, qualitative input'],
		features: ['Short-form video capture', 'Optional prompts to guide contributions'],
		whatYoudGet: ['Video contributions', 'Consent-managed testimony']
	}
};

/**
 * A palette entry that has no backing workflow tool, so "+ Add this step" creates a
 * conversation Event (e.g. a live video conference) instead of a workflow step. It
 * carries the same display fields as {@link ToolMeta} but no {@link CreationKey}; the
 * absence of `creationKey` is what {@link isEventPaletteItem} keys off. See CONTEXT.md
 * ("Online video conference" is display-only in templates until a tool backs it).
 */
export type EventPaletteMeta = {
	/** Palette-only identifier. Not a ToolConfig discriminant, so it never resolves via {@link toolMeta}. */
	type: 'videoconference';
	displayName: string;
	tagline: string;
	icon: typeof Icon;
	infoSlug: string;
	estimatedMinutes: number;
	description: string;
	bestFor: string[];
	features: string[];
	whatYoudGet: string[];
};

/** An entry in the add-step palette: a real tool, or an event stand-in. */
export type PaletteItem = ToolMeta | EventPaletteMeta;

/** Narrows a palette entry to the event stand-in (adds an Event, not a workflow step). */
export function isEventPaletteItem(item: PaletteItem): item is EventPaletteMeta {
	return !('creationKey' in item);
}

const VIDEO_CONFERENCE: EventPaletteMeta = {
	type: 'videoconference',
	displayName: 'Online video conference',
	tagline: 'Facilitate live video meetings for real-time collaboration and discussion.',
	icon: Video,
	infoSlug: 'online_group_conversation',
	estimatedMinutes: 60,
	description: 'Facilitate live video meetings for real-time collaboration and discussion.',
	bestFor: ['Live workshops and discussion', 'Presentations, demonstrations, and training'],
	features: [
		'Breakout rooms for small group discussions',
		'Supports interactive facilitation in a live setting'
	],
	whatYoudGet: [
		'Transcription of main and breakout rooms',
		'Summary and themes of ideas emerged from the discussion'
	]
};

/**
 * Order the entries appear in the left palette. Follows the finalised Figma palette
 * order (Topic onboarding, Participant-led poll, Survey, Online video conference,
 * Individual view exploration), with the remaining working-but-Figma-omitted tools
 * appended so nothing regresses (see CONTEXT.md / grilling notes).
 */
export const PALETTE_TOOLS: PaletteItem[] = [
	TOOL_META.learn,
	TOOL_META.polis,
	TOOL_META.heyform,
	// VIDEO_CONFERENCE, // commenting this out until we decide the behaviour we want
	TOOL_META.thinkingspace,
	TOOL_META.prioritization,
	TOOL_META.elicitationbot,
	TOOL_META.stories
];

export function toolMeta(type: string | undefined | null): ToolMeta | undefined {
	if (!type) return undefined;
	return TOOL_META[type as ToolType];
}

export function toolInfoUrl(type: string | undefined | null): string {
	const meta = toolMeta(type);
	return meta ? `/admin/info/tools/${meta.infoSlug}` : '/admin/info/tools';
}

/**
 * Data protocol ladder (least -> most open). Only Confidential and Restricted are
 * currently backed (mapped onto the `request_user_share_permission` boolean); the
 * other two render disabled pending a real enum column. See CONTEXT.md.
 */
export type DataProtocol = 'confidential' | 'restricted' | 'collaborative' | 'open';

export const DATA_PROTOCOLS: {
	value: DataProtocol;
	label: string;
	blurb: string;
	enabled: boolean;
}[] = [
	{
		value: 'confidential',
		label: 'Confidential',
		blurb: 'User data will not be shared with anyone.',
		enabled: true
	},
	{
		value: 'restricted',
		label: 'Restricted',
		blurb: 'User data will be shared with the organiser only.',
		enabled: true
	},
	{
		value: 'collaborative',
		label: 'Collaborative',
		blurb: 'User data will be shared with the organiser and other participants.',
		enabled: false
	},
	{
		value: 'open',
		label: 'Open',
		blurb: 'User data will be shared with everyone.',
		enabled: false
	}
];

/** Map the existing boolean onto the two backed protocol levels. */
export function protocolFromBool(share: boolean): DataProtocol {
	return share ? 'restricted' : 'confidential';
}
export function boolFromProtocol(p: DataProtocol): boolean {
	return p === 'restricted';
}
