<script lang="ts">
	import { resolve } from '$app/paths';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { BookOpenText, Building2, Images, LayoutGrid, Mail, Map, Shield } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';

	type Props = { path: string; isSiteAdmin: boolean; isSuperAdmin: boolean };
	let { path, isSiteAdmin, isSuperAdmin }: Props = $props();

	/** `section` is the path prefix the link counts as active under, when wider than `href`. */
	type WorkspaceLink = { label: string; href: string; icon: ComponentType; section?: string };

	const conversationsHref = resolve('/admin/conversations');

	let links = $derived<WorkspaceLink[]>([
		{ label: 'Conversations', href: conversationsHref, icon: LayoutGrid },
		...(isSiteAdmin
			? [
					{ label: 'Media library', href: resolve('/admin/media-library'), icon: Images },
					{
						label: 'Email templates',
						href: resolve('/admin/email-template-configs'),
						icon: Mail
					}
				]
			: []),
		{ label: 'Organisations', href: resolve('/admin/organizations'), icon: Building2 },
		...(isSuperAdmin
			? [
					{ label: 'Regions', href: resolve('/admin/regions'), icon: Map },
					{
						label: 'System roles',
						href: resolve('/(admin)/admin/system-roles'),
						icon: Shield
					}
				]
			: []),
		{
			label: 'Handbook',
			href: resolve('/admin/info/welcome'),
			section: resolve('/admin/info'),
			icon: BookOpenText
		}
	]);

	// "Conversations" lights up on the list, not inside a conversation.
	function isActive({ href, section = href }: WorkspaceLink): boolean {
		if (href === conversationsHref) return path.replace(/\/$/, '') === href;
		return path === section || path.startsWith(`${section}/`);
	}
</script>

<!-- Every href in `links` comes from resolve() above. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<SideBar.Menu>
	{#each links as link (link.href)}
		<SideBar.MenuItem>
			<SideBar.MenuButton
				isActive={isActive(link)}
				tooltipContent={link.label}
				class="text-base"
			>
				{#snippet child({ props })}
					<a {...props} href={link.href}>
						<link.icon />
						<span class="group-data-[collapsible=icon]:hidden">{link.label}</span>
					</a>
				{/snippet}
			</SideBar.MenuButton>
		</SideBar.MenuItem>
	{/each}
</SideBar.Menu>
