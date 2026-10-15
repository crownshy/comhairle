import type { ComponentType } from 'svelte';
import type { ConversationDto } from '@crownshy/api-client/api';
import {
	conversationSections,
	type ConversationSection,
	type SectionStage
} from '$lib/config/conversation-steps';

export type StageState = 'done' | 'current' | 'locked';

export type SectionLock = 'until-launch' | 'admin-only';

export const sectionLockLabels: Record<SectionLock, string> = {
	'until-launch': 'Available after launch',
	'admin-only': 'Admins only'
};

export type SectionLink = {
	name: string;
	path: string;
	href: string;
	icon: ComponentType;
	active: boolean;
	/** Why the link can't be opened, or null when it can. */
	lock: SectionLock | null;
};

export type ConversationStage = {
	id: SectionStage | 'launch';
	label: string;
	state: StageState;
	sections: SectionLink[];
};

export type StageConversation = Pick<ConversationDto, 'id' | 'isLive' | 'isComplete'>;

type StageInput = {
	conversation: StageConversation;
	pathname: string;
	canAdmin: boolean;
};

export function conversationStages({
	conversation,
	pathname,
	canAdmin
}: StageInput): ConversationStage[] {
	const base = `/admin/conversations/${conversation.id}`;
	const sectionsIn = (stage: SectionStage) =>
		conversationSections
			.filter((section) => section.stage === stage)
			.map((section) => sectionLink(section, base, pathname, conversation.isLive, canAdmin));
	const live = conversation.isLive;

	return [
		{
			id: 'set-up',
			label: 'Set up',
			state: live ? 'done' : 'current',
			sections: sectionsIn('set-up')
		},
		{
			id: 'launch',
			label: live ? 'Launched' : 'Launch',
			state: live ? 'done' : 'current',
			sections: []
		},
		{
			id: 'engage',
			label: 'Engage',
			state: !live ? 'locked' : conversation.isComplete ? 'done' : 'current',
			sections: sectionsIn('engage')
		},
		{
			id: 'results',
			label: 'Results',
			state: live ? 'current' : 'locked',
			sections: sectionsIn('results')
		}
	];
}

function sectionLink(
	section: ConversationSection,
	base: string,
	pathname: string,
	isLive: boolean,
	canAdmin: boolean
): SectionLink {
	const href = `${base}/${section.path}`;
	return {
		name: section.name,
		path: section.path,
		href,
		icon: section.icon,
		active: pathname === href || pathname.startsWith(`${href}/`),
		lock: sectionLock(section, isLive, canAdmin)
	};
}

function sectionLock(
	section: ConversationSection,
	isLive: boolean,
	canAdmin: boolean
): SectionLock | null {
	if (section.requiresLive && !isLive) return 'until-launch';
	if (section.requiresAdmin && !canAdmin) return 'admin-only';
	return null;
}
