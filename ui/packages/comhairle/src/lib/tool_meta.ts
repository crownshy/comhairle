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

export type ToolMeta = {
	/** ToolConfig discriminant. */
	type: ToolType;
	/** Admin-facing label (Figma display name). Distinct from the internal creation key. */
	displayName: string;
	/** One-line tooltip / palette description. */
	tagline: string;
	/** Icon shown on cards and the palette. */
	icon: typeof Icon;
	/** Route key under /admin/info/tools/<infoKey>. */
	infoKey: string;
	/** Key passed to the add-step switch (see {@link import('./createWorkflowStep').createWorkflowStep}). */
	creationKey: string;
	/**
	 * Hardcoded typical duration in minutes shown on the "Estimated time" pill.
	 * Placeholder until a real per-step `estimated_minutes` column lands (see CONTEXT.md).
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
 * detail-panel content. `learn` uses the finalised Figma copy; the other six are
 * PLACEHOLDER copy derived from each tool's tagline and still need product/design
 * sign-off before this ships.
 */
export const TOOL_META: Record<ToolType, ToolMeta> = {
	learn: {
		type: 'learn',
		displayName: 'Topic onboarding',
		tagline:
			'Present participants with information to help them learn about the topic at hand.',
		icon: BookOpen,
		infoKey: 'learn',
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
		infoKey: 'thinking_space',
		creationKey: 'Thinking Space',
		estimatedMinutes: 12,
		// PLACEHOLDER copy — needs product/design review.
		description:
			'Help participants explore their own views before contributing. A guided space asks non-leading coaching questions that broaden and deepen what a participant thinks, one follow-up at a time.',
		bestFor: [
			'Helping participants form considered views',
			'Warming up before a poll or discussion'
		],
		features: [
			'Adaptive follow-up questions',
			'Participants review and edit before submitting'
		],
		whatYoudGet: ['Reflective written responses', 'Themes across participants']
	},
	polis: {
		type: 'polis',
		displayName: 'Participant-led poll',
		tagline:
			"Show participants others' views and vote 'Agree' 'Disagree' or 'Pass' if unable to decide. Participants can also make their own views for others to vote on.",
		icon: MessagesSquare,
		infoKey: 'polis',
		creationKey: 'Polis',
		estimatedMinutes: 12,
		// PLACEHOLDER copy — needs product/design review.
		description:
			'Surface the range of views in your community. Participants vote Agree, Disagree, or Pass on statements, and can add their own for others to vote on, revealing where consensus and division lie.',
		bestFor: [
			'Finding common ground on divisive topics',
			"Surfacing views you didn't know to ask about"
		],
		features: [
			'Participant-submitted statements',
			'Real-time consensus and opinion-group analysis'
		],
		whatYoudGet: ['Consensus statements', 'Opinion-group breakdown']
	},
	heyform: {
		type: 'heyform',
		displayName: 'Survey',
		tagline: 'Ask participants a series of pre-planned questions.',
		icon: ListChecks,
		infoKey: 'heyform',
		creationKey: 'Survey',
		estimatedMinutes: 9,
		// PLACEHOLDER copy — needs product/design review.
		description:
			'Ask participants a series of pre-planned questions. Mix multiple choice, rating, and open-text questions to gather structured, comparable responses.',
		bestFor: ['Structured feedback', 'Demographic or screening questions'],
		features: ['Multiple question types', 'Required and optional questions'],
		whatYoudGet: ['Per-question response breakdown', 'Exportable responses']
	},
	prioritization: {
		type: 'prioritization',
		displayName: 'Proposal prioritisation',
		tagline:
			'Collect a set of proposals and have participants score each one against a shared set of questions.',
		icon: ListOrdered,
		infoKey: 'prioritization',
		creationKey: 'Prioritization',
		estimatedMinutes: 10,
		// PLACEHOLDER copy — needs product/design review.
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
		infoKey: 'elicitation_bot',
		creationKey: 'Elicitation Bot',
		estimatedMinutes: 10,
		// PLACEHOLDER copy — needs product/design review.
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
		infoKey: 'lived_experience',
		creationKey: 'Lived Experience',
		estimatedMinutes: 8,
		// PLACEHOLDER copy — needs product/design review.
		description:
			'Let participants record short videos sharing their lived experience, capturing context and emotion that text alone can miss.',
		bestFor: ['Gathering personal testimony', 'Human-centred, qualitative input'],
		features: ['Short-form video capture', 'Optional prompts to guide contributions'],
		whatYoudGet: ['Video contributions', 'Consent-managed testimony']
	}
};

/**
 * Order the tools appear in the left palette. Matches the Figma palette (minus the
 * unbacked "Video call"), with the two Figma-omitted-but-working tools appended so
 * nothing regresses (see CONTEXT.md / grilling notes).
 */
export const PALETTE_ORDER: ToolType[] = [
	'learn',
	'thinkingspace',
	'polis',
	'heyform',
	'prioritization',
	'elicitationbot',
	'stories'
];

export const PALETTE_TOOLS: ToolMeta[] = PALETTE_ORDER.map((t) => TOOL_META[t]);

export function toolMeta(type: string | undefined | null): ToolMeta | undefined {
	if (!type) return undefined;
	return TOOL_META[type as ToolType];
}

export function toolInfoUrl(type: string | undefined | null): string {
	const meta = toolMeta(type);
	return meta ? `/admin/info/tools/${meta.infoKey}` : '/admin/info/tools';
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
