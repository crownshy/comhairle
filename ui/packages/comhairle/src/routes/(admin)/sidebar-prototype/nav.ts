// PROTOTYPE, delete when the sidebar question is settled. See NOTES.md.
import {
	BookOpenText,
	Building2,
	CalendarDays,
	ChartNoAxesColumn,
	Images,
	LayoutGrid,
	Mail,
	Megaphone,
	NotebookText,
	Settings2,
	UsersRound,
	Workflow
} from 'lucide-svelte';
import type { ComponentType } from 'svelte';
import { conversationSections } from '$lib/config/conversation-steps';

export type NavItem = { label: string; href: string; icon: ComponentType };

export const workspaceNav: NavItem[] = [
	{ label: 'Conversations', href: '/admin', icon: LayoutGrid },
	{ label: 'Organisations', href: '/admin/organizations', icon: Building2 }
];

export const libraryNav: NavItem[] = [
	{ label: 'Email templates', href: '/admin/email-template-configs', icon: Mail },
	{ label: 'Media library', href: '/admin/media-library', icon: Images }
];

// The real config reuses Database for two sections; distinct icons matter once the list is the nav.
const sectionIcons: Record<string, ComponentType> = {
	configure: Settings2,
	design: Workflow,
	'learning-assistant': BookOpenText,
	events: CalendarDays,
	invites: UsersRound,
	notifications: Megaphone,
	monitor: ChartNoAxesColumn,
	report: NotebookText
};

export type SectionLink = {
	label: string;
	path: string;
	href: string;
	icon: ComponentType;
	locked: boolean;
	active: boolean;
};

export type ConversationLike = { id: string; title: string; isLive: boolean; isComplete: boolean };

export function sectionLinks(conversation: ConversationLike, pathname: string): SectionLink[] {
	const base = `/admin/conversations/${conversation.id}`;
	return conversationSections.map((section) => {
		const href = `${base}/${section.path}`;
		return {
			label: section.name,
			path: section.path,
			href,
			icon: sectionIcons[section.path] ?? section.icon,
			locked: !!section.requiresLive && !conversation.isLive,
			active: pathname === href || pathname.startsWith(`${href}/`)
		};
	});
}

export const phases = [
	{ id: 'build', label: 'Build', paths: ['configure', 'design', 'learning-assistant'] },
	{ id: 'run', label: 'Run', paths: ['events', 'invites', 'notifications'] },
	{ id: 'results', label: 'Results', paths: ['monitor', 'report'] }
] as const;

export function groupByPhase(links: SectionLink[]) {
	return phases.map((phase) => ({
		...phase,
		links: phase.paths
			.map((path) => links.find((link) => link.path === path))
			.filter((link): link is SectionLink => !!link)
	}));
}

export function conversationStatus(conversation: ConversationLike): 'Draft' | 'Live' | 'Closed' {
	if (conversation.isComplete) return 'Closed';
	return conversation.isLive ? 'Live' : 'Draft';
}

export function isNavActive(href: string, pathname: string): boolean {
	if (href === '/admin') return pathname === '/admin' || pathname === '/admin/';
	return pathname === href || pathname.startsWith(`${href}/`);
}

export type StepLike = { id: string; name: string; stepOrder: number };
