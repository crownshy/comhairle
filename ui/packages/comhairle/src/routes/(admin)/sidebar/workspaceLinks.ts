import type { ComponentType } from 'svelte';
import { resolve } from '$app/paths';
import { Building2, Images, LayoutGrid, Mail, Map, Shield } from 'lucide-svelte';

export type WorkspaceLink = {
	label: string;
	href: string;
	icon: ComponentType;
};

export type WorkspaceGroup = {
	/** Left out for the top group, which needs no heading. */
	label?: string;
	links: WorkspaceLink[];
};

type WorkspaceRoles = { isSiteAdmin: boolean; isSuperAdmin: boolean };

export function workspaceGroups({ isSiteAdmin, isSuperAdmin }: WorkspaceRoles): WorkspaceGroup[] {
	const groups: WorkspaceGroup[] = [
		{
			links: [
				{
					label: 'Conversations',
					href: resolve('/admin/conversations'),
					icon: LayoutGrid
				}
			]
		}
	];

	if (isSiteAdmin) {
		groups.push({
			label: 'Library',
			links: [
				{ label: 'Media library', href: resolve('/admin/media-library'), icon: Images },
				{
					label: 'Email templates',
					href: resolve('/admin/email-template-configs'),
					icon: Mail
				}
			]
		});
	}

	groups.push({
		label: 'Administration',
		links: [
			{ label: 'Organisations', href: resolve('/admin/organizations'), icon: Building2 },
			...(isSuperAdmin
				? [
						{ label: 'Regions', href: resolve('/admin/regions'), icon: Map },
						{
							label: 'System roles',
							href: resolve('/admin/system-roles'),
							icon: Shield
						}
					]
				: [])
		]
	});

	return groups;
}

export function isLinkActive(link: WorkspaceLink, pathname: string): boolean {
	return pathname === link.href || pathname.startsWith(`${link.href}/`);
}
