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
	activeStatus: NavLinkActiveStatus;
}

export enum NavLinkActiveStatus {
	Launch = 'launch',
	PreLaunch = 'pre-launch',
	Both = 'both'
}

export const conversationSteps: ConversationStep[] = [
	{
		name: 'Configure',
		path: 'configure',
		icon: TerminalSquare,
		activeStatus: NavLinkActiveStatus.Both
	},
	{ name: 'Design', path: 'design', icon: Pencil, activeStatus: NavLinkActiveStatus.PreLaunch },
	{
		name: 'Knowledge base',
		path: 'knowledge-base',
		icon: Database,
		activeStatus: NavLinkActiveStatus.PreLaunch
	},
	{
		name: 'Recruit',
		path: 'invites',
		icon: UsersRound,
		activeStatus: NavLinkActiveStatus.Launch
	},
	{
		name: 'Monitor',
		path: 'monitor',
		icon: Binoculars,
		activeStatus: NavLinkActiveStatus.Launch
	},
	{
		name: 'Moderate',
		path: 'moderate',
		icon: UsersRound,
		activeStatus: NavLinkActiveStatus.Launch
	},
	{ name: 'Notify', path: 'notifications', icon: Bell, activeStatus: NavLinkActiveStatus.Launch },
	{ name: 'Report', path: 'report', icon: NotebookText, activeStatus: NavLinkActiveStatus.Launch }
];
