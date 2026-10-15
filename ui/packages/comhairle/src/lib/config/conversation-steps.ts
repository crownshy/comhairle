import {
	SlidersHorizontal,
	GitBranch,
	BookOpen,
	CalendarDays,
	UsersRound,
	Bell,
	Binoculars,
	NotebookText,
	ShieldCheck
} from 'lucide-svelte';
import type { ComponentType } from 'svelte';

/** The Conversation Stages that hold sections. Launch is a milestone with no sections. */
export type SectionStage = 'set-up' | 'engage' | 'results';

export interface ConversationSection {
	name: string;
	path: string;
	icon: ComponentType;
	stage: SectionStage;
	/**
	 * If true, the section is disabled until the conversation is launched. Not for
	 * edit protection, but because the section's content depends on
	 * participant activity that doesn't exist yet (recruiting, monitoring,
	 * moderation queues, notifications, reports). See ADR-0001.
	 */
	requiresLive?: boolean;
	/** Only conversation Admins can open it. */
	requiresAdmin?: boolean;
	/** Only listed when the conversation has a Polis step to moderate. */
	requiresPolis?: boolean;
}

export const conversationSections: ConversationSection[] = [
	{ name: 'Configure', path: 'configure', icon: SlidersHorizontal, stage: 'set-up' },
	{ name: 'Process design', path: 'design', icon: GitBranch, stage: 'set-up' },
	{ name: 'Learning Assistant', path: 'learning-assistant', icon: BookOpen, stage: 'set-up' },
	{ name: 'Events', path: 'events', icon: CalendarDays, stage: 'set-up' },
	{ name: 'Recruit', path: 'invites', icon: UsersRound, stage: 'engage', requiresLive: true },
	{
		name: 'Notify',
		path: 'notifications',
		icon: Bell,
		stage: 'engage',
		requiresLive: true,
		requiresAdmin: true
	},
	{
		name: 'Moderate',
		path: 'moderate',
		icon: ShieldCheck,
		stage: 'engage',
		requiresLive: true,
		requiresPolis: true
	},
	{ name: 'Monitor', path: 'monitor', icon: Binoculars, stage: 'engage', requiresLive: true },
	{ name: 'Report', path: 'report', icon: NotebookText, stage: 'results', requiresLive: true }
];
