import type { ComponentType } from 'svelte';
import { BookOpenText, Building2, Images, LayoutGrid, Mail, Map, Shield } from 'lucide-svelte';

export type WorkspaceLink = {
	label: string;
	description: string;
	href: string;
	icon: ComponentType;
};

type WorkspaceRoles = { isSiteAdmin: boolean; isSuperAdmin: boolean };

export function workspaceLinks({ isSiteAdmin, isSuperAdmin }: WorkspaceRoles): WorkspaceLink[] {
	return [
		{
			label: 'Conversations',
			description: 'Every conversation you own or have been shared',
			href: '/admin/conversations',
			icon: LayoutGrid
		},
		...(isSiteAdmin
			? [
					{
						label: 'Media library',
						description: 'Images and files used across conversations',
						href: '/admin/media-library',
						icon: Images
					},
					{
						label: 'Email templates',
						description: 'Custom emails sent to participants',
						href: '/admin/email-template-configs',
						icon: Mail
					}
				]
			: []),
		{
			label: 'Organisations',
			description: 'Organisations you belong to and their members',
			href: '/admin/organizations',
			icon: Building2
		},
		...(isSuperAdmin
			? [
					{
						label: 'Regions',
						description: 'Regions participants can choose from',
						href: '/admin/regions',
						icon: Map
					},
					{
						label: 'System roles',
						description: 'Who can administer the platform',
						href: '/admin/system-roles',
						icon: Shield
					}
				]
			: []),
		{
			label: 'Handbook',
			description: 'Guides for running a conversation',
			href: '/admin/info/welcome',
			icon: BookOpenText
		}
	];
}

/** Breadcrumb labels for workspace sections, keyed by the path segment after /admin. */
const SECTION_LABELS: Record<string, string> = {
	conversations: 'Conversations',
	'media-library': 'Media library',
	'email-template-configs': 'Email templates',
	organizations: 'Organisations',
	regions: 'Regions',
	'system-roles': 'System roles'
};

export type Crumb = { label: string; href?: string };

/**
 * The trail after "Workspace" for a workspace page. The section links back to its list when
 * you're on a page below it; the last crumb is the current page and has no link.
 */
export function workspaceCrumbs(pathname: string): Crumb[] {
	const [section, ...rest] = pathname
		.replace(/^\/admin\/?/, '')
		.split('/')
		.filter(Boolean);
	const label = section && SECTION_LABELS[section];
	if (!label) return [];
	if (rest.length === 0) return [{ label }];

	const sectionCrumb = { label, href: `/admin/${section}` };
	return rest.at(-1) === 'new' ? [sectionCrumb, { label: 'New' }] : [sectionCrumb];
}
