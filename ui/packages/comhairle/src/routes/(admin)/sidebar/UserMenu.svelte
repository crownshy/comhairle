<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import * as SideBar from '$lib/components/ui/sidebar';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronsUpDown, Home, Info, LogOut, Settings } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { UserDto } from '@crownshy/api-client/api';
	import { userInitials } from '$lib/utils';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import { notifications } from '$lib/notifications.svelte';
	import { m } from '$lib/paraglide/messages';

	let { user }: { user: UserDto } = $props();
	const sidebar = useSidebar();

	async function signOut() {
		const logout = await tryCatchAsync(async () => {
			await apiClient.LogoutUser(undefined);
			await goto(resolve('/'), { invalidate: [key('user')] });
		});
		if (logout.err) {
			notifications.send({ priority: 'ERROR', message: m.logout_failed() });
		}
	}
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
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a {...props} href={resolve('/settings')}>
							<Settings class="size-4" /> Settings
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a {...props} href={resolve('/')}>
							<Home class="size-4" /> Back to home
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a {...props} href={resolve('/about')}>
							<Info class="size-4" /> About Comhairle
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Separator />
				<DropdownMenu.Item onclick={signOut}>
					<LogOut class="size-4" /> Sign out
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</SideBar.MenuItem>
</SideBar.Menu>
