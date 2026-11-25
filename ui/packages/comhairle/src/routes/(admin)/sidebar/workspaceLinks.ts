import type { Component } from 'svelte';
import type { IconProps } from '@lucide/svelte';
import { resolve } from '$app/paths';
import Building2 from '@lucide/svelte/icons/building-2';
import ClipboardList from '@lucide/svelte/icons/clipboard-list';
import FlaskConical from '@lucide/svelte/icons/flask-conical';
import Images from '@lucide/svelte/icons/images';
import LayoutGrid from '@lucide/svelte/icons/layout-grid';
import Mail from '@lucide/svelte/icons/mail';
import Map from '@lucide/svelte/icons/map';
import Shield from '@lucide/svelte/icons/shield';

export type WorkspaceLink = {
	label: string;
	href: string;
	icon: Component<IconProps>;
};

export type WorkspaceGroup = {
	/** Left out for the top group, which needs no heading. */
	label?: string;
	links: WorkspaceLink[];
};

type WorkspaceRoles = { isSiteAdmin: boolean; isSuperAdmin: boolean; isModeratorOnly?: boolean };

export function workspaceGroups({
	isSiteAdmin,
	isSuperAdmin,
	isModeratorOnly = false
}: WorkspaceRoles): WorkspaceGroup[] {
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
	if (isModeratorOnly) return groups;

	if (isSiteAdmin) {
		groups.push({
			label: 'Library',
			links: [
				{ label: 'Media library', href: resolve('/admin/media-library'), icon: Images },
				{
					label: 'Email templates',
					href: resolve('/admin/email-template-configs'),
					icon: Mail
				},
				{
					label: 'Demographic forms',
					href: resolve('/admin/demographic-forms'),
					icon: ClipboardList
				},
				{
					label: 'Hybrid works (prototype)',
					href: resolve('/admin/demographic-forms/prototype-conversation/design'),
					icon: FlaskConical
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
