<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import { cn } from '$lib/utils';
	import { isLinkActive, type WorkspaceGroup } from './workspaceLinks';
	import { LINK_ROW, LINK_ROW_IDLE, ROW_ACTIVE, ROW_HOVER } from './sidebarRows';

	type Props = { groups: WorkspaceGroup[]; path: string };
	let { groups, path }: Props = $props();

	let links = $derived(groups.flatMap((group) => group.links));
</script>

<!-- Link hrefs come from workspaceGroups, which builds /admin paths. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<nav aria-label="Workspace" class="flex flex-col gap-5 pr-4 group-data-[collapsible=icon]:hidden">
	{#each groups as group (group.label ?? '')}
		<div class="flex flex-col gap-0.5">
			{#if group.label}
				<h2 class="text-sidebar-foreground/60 px-2 pb-1 text-sm font-medium">
					{group.label}
				</h2>
			{/if}
			<ul class="flex flex-col gap-0.5">
				{#each group.links as link (link.href)}
					{@const active = isLinkActive(link, path)}
					<li>
						<a
							href={link.href}
							aria-current={active ? 'page' : undefined}
							class={cn(LINK_ROW, active ? ROW_ACTIVE : [LINK_ROW_IDLE, ROW_HOVER])}
						>
							<link.icon class="size-4 shrink-0" />
							<span class="flex-1 truncate">{link.label}</span>
						</a>
					</li>
				{/each}
			</ul>
		</div>
	{/each}
</nav>

<!-- Collapsed: one icon per link, in the same order. -->
<SideBar.Menu class="hidden items-center group-data-[collapsible=icon]:flex">
	{#each links as link (link.href)}
		<SideBar.MenuItem>
			<SideBar.MenuButton isActive={isLinkActive(link, path)} tooltipContent={link.label}>
				{#snippet child({ props })}
					<a {...props} href={link.href}>
						<link.icon />
						<span class="sr-only">{link.label}</span>
					</a>
				{/snippet}
			</SideBar.MenuButton>
		</SideBar.MenuItem>
	{/each}
</SideBar.Menu>
