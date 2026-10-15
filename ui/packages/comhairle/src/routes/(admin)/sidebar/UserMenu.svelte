<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronsUpDown } from 'lucide-svelte';
	import type { UserDto } from '@crownshy/api-client/api';
	import { userInitials } from '$lib/utils';
	import UserMenuItems from '../account/UserMenuItems.svelte';

	let { user }: { user: UserDto } = $props();
	const sidebar = useSidebar();
</script>

<SideBar.Menu>
	<SideBar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<SideBar.MenuButton size="lg" {...props}>
						<Avatar.Root class="size-8 rounded-lg">
							{#if user?.avatarUrl}
								<Avatar.Image src={user.avatarUrl} alt={user.username} />
							{/if}
							<Avatar.Fallback class="rounded-lg">
								{userInitials(user?.username ?? '')}
							</Avatar.Fallback>
						</Avatar.Root>
						<span
							class="flex min-w-0 flex-1 flex-col text-left leading-tight group-data-[collapsible=icon]:hidden"
						>
							<span class="truncate text-base font-semibold">{user?.username}</span>
							{#if user?.email}
								<span class="text-sidebar-foreground/70 truncate text-sm">
									{user.email}
								</span>
							{/if}
						</span>
						<ChevronsUpDown
							class="ml-auto size-4 group-data-[collapsible=icon]:hidden"
						/>
					</SideBar.MenuButton>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				side={sidebar.isMobile ? 'top' : 'right'}
				align="end"
				class="w-56"
			>
				<UserMenuItems />
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</SideBar.MenuItem>
</SideBar.Menu>
