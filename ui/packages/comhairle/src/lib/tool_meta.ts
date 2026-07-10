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
	/** Key passed to the add-step switch (see design/+layout.svelte `addStep`). */
	creationKey: string;
	/**
	 * Hardcoded typical duration in minutes shown on the "Estimated time" pill.
	 * Placeholder until a real per-step `estimated_minutes` column lands (see CONTEXT.md).
	 */
	estimatedMinutes: number;
};

/**
 * Single source of truth for how each engagement tool is presented in the admin UI.
 * Keyed by the ToolConfig discriminant.
 */
export const TOOL_META: Record<ToolType, ToolMeta> = {
	learn: {
		type: 'learn',
		displayName: 'Rich content page',
		tagline:
			'Present participants with information to help them learn about the topic at hand.',
		icon: BookOpen,
		infoKey: 'learn',
		creationKey: 'Learn',
		estimatedMinutes: 10
	},
	thinkingspace: {
		type: 'thinkingspace',
		displayName: 'Thinking space',
		tagline:
			'Help participants explore their views by asking them non-leading coaching questions that broaden and deepen their views.',
		icon: Bot,
		infoKey: 'thinking_space',
		creationKey: 'Thinking Space',
		estimatedMinutes: 12
	},
	polis: {
		type: 'polis',
		displayName: 'Wiki Poll (Pol.is)',
		tagline:
			"Show participants others' views and vote 'Agree' 'Disagree' or 'Pass' if unable to decide. Participants can also make their own views for others to vote on.",
		icon: MessagesSquare,
		infoKey: 'polis',
		creationKey: 'Polis',
		estimatedMinutes: 12
	},
	heyform: {
		type: 'heyform',
		displayName: 'Survey',
		tagline: 'Ask participants a series of pre-planned questions.',
		icon: ListChecks,
		infoKey: 'heyform',
		creationKey: 'Survey',
		estimatedMinutes: 9
	},
	prioritization: {
		type: 'prioritization',
		displayName: 'Prioritisation tool',
		tagline:
			'Collect a set of proposals and have participants score each one against a shared set of questions.',
		icon: ListOrdered,
		infoKey: 'prioritization',
		creationKey: 'Prioritization',
		estimatedMinutes: 10
	},
	elicitationbot: {
		type: 'elicitationbot',
		displayName: 'Elicitation Bot',
		tagline:
			'Help participants refine and capture their views through an AI bot mediated interaction.',
		icon: Bot,
		infoKey: 'elicitation_bot',
		creationKey: 'Elicitation Bot',
		estimatedMinutes: 10
	},
	stories: {
		type: 'stories',
		displayName: 'Lived Experience',
		tagline: 'Let users record short videos of their lived experience.',
		icon: Video,
		infoKey: 'lived_experience',
		creationKey: 'Lived Experience',
		estimatedMinutes: 8
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
