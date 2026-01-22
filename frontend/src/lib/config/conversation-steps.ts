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
}

export const conversationSteps: ConversationStep[] = [
	{ name: 'Configure', path: 'configure', icon: TerminalSquare },
	{ name: 'Design', path: 'design', icon: Pencil },
	{ name: 'Recruit', path: 'invites', icon: UsersRound },
	{ name: 'Monitor', path: 'monitor', icon: Binoculars },
	{ name: 'Moderate', path: 'moderate', icon: UsersRound },
	{ name: 'Knowledge base', path: 'knowledge-base', icon: Database },
	{ name: 'Notify', path: 'notifications', icon: Bell },
	{ name: 'Report', path: 'report', icon: NotebookText }
];
