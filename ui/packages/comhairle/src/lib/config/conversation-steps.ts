import {
	TerminalSquare,
	Pencil,
	UsersRound,
	Binoculars,
	Database,
	Bell,
	NotebookText
} from 'lucide-svelte';
import type { ComponentType } from 'svelte';

export interface ConversationSection {
	name: string;
	path: string;
	icon: ComponentType;
	/**
	 * If true, the section is disabled until the conversation is launched —
	 * not for edit protection, but because the section's content depends on
	 * participant activity that doesn't exist yet (recruiting, monitoring,
	 * moderation queues, notifications, reports). See ADR-0001.
	 */
	requiresLive?: boolean;
}

export const conversationSections: ConversationSection[] = [
	{ name: 'Configure', path: 'configure', icon: TerminalSquare },
	{ name: 'Workflow', path: 'design', icon: Pencil },
	{ name: 'Knowledge base', path: 'knowledge-base', icon: Database },
	{ name: 'Events', path: 'events', icon: Database },
	{ name: 'Recruit', path: 'invites', icon: UsersRound, requiresLive: true },
	{ name: 'Monitor', path: 'monitor', icon: Binoculars, requiresLive: true },
	{ name: 'Moderate', path: 'moderate', icon: UsersRound, requiresLive: true },
	{ name: 'Notify', path: 'notifications', icon: Bell, requiresLive: true },
	{ name: 'Report', path: 'report', icon: NotebookText, requiresLive: true }
];
