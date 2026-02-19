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

export interface ConversationStep {
	name: string;
	path: string;
	icon: ComponentType;
	activeOnLive: boolean;
}

export const conversationSteps: ConversationStep[] = [
	{ name: 'Configure', path: 'configure', icon: TerminalSquare, activeOnLive: true },
	{ name: 'Design', path: 'design', icon: Pencil, activeOnLive: false },
	{ name: 'Knowledge base', path: 'knowledge-base', icon: Database, activeOnLive: false },
	{ name: 'Recruit', path: 'invites', icon: UsersRound, activeOnLive: true },
	{ name: 'Monitor', path: 'monitor', icon: Binoculars, activeOnLive: true },
	{ name: 'Moderate', path: 'moderate', icon: UsersRound, activeOnLive: true },
	{ name: 'Notify', path: 'notifications', icon: Bell, activeOnLive: true },
	{ name: 'Report', path: 'report', icon: NotebookText, activeOnLive: true }
];
