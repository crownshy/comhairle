<script lang="ts">
	import * as SideBar from '$lib/components/ui/sidebar';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronsUpDown } from 'lucide-svelte';
	import type { UserDto } from '@crownshy/api-client/api';
	import UserMenuItems from '../account/UserMenuItems.svelte';
	import UserAvatar from '$lib/components/UserAvatar.svelte';

	interface Props {
		user: UserDto;
	}
	let { user }: Props = $props();
	const sidebar = useSidebar();
</script>

<SideBar.Menu>
	<SideBar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<SideBar.MenuButton size="lg" {...props}>
						<UserAvatar
							username={user.username}
							avatarUrl={user.avatarUrl}
							class="rounded-lg"
						/>
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
