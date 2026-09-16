<script lang="ts">
	// PROTOTYPE. Workspace-level groups: no conversation list, the Conversations page owns that.
	import * as SideBar from '$lib/components/ui/sidebar';
	import { page } from '$app/state';
	import { isNavActive, libraryNav, workspaceNav, type NavItem } from './nav';
</script>

{#snippet group(label: string, items: NavItem[])}
	<SideBar.Group>
		<SideBar.GroupLabel>{label}</SideBar.GroupLabel>
		<SideBar.Menu>
			{#each items as item (item.href)}
				<SideBar.MenuItem>
					<SideBar.MenuButton
						isActive={isNavActive(item.href, page.url.pathname)}
						tooltipContent={item.label}
					>
						{#snippet child({ props })}
							<a href={item.href} {...props}>
								<item.icon />
								<span>{item.label}</span>
							</a>
						{/snippet}
					</SideBar.MenuButton>
				</SideBar.MenuItem>
			{/each}
		</SideBar.Menu>
	</SideBar.Group>
{/snippet}

{@render group('Workspace', workspaceNav)}
{@render group('Library', libraryNav)}
