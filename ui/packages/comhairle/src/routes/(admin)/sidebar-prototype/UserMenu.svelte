<script lang="ts">
	// PROTOTYPE. Account + rarely-used links folded into one footer menu.
	import * as SideBar from '$lib/components/ui/sidebar';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ChevronsUpDown, ExternalLink, Info, Settings } from 'lucide-svelte';
	import { userInitials } from '$lib/utils';
	import type { UserDto } from '@crownshy/api-client/api';

	let { user }: { user: UserDto } = $props();
</script>

<SideBar.Menu>
	<SideBar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<SideBar.MenuButton size="lg" {...props} class="h-12">
						<Avatar.Root class="size-8 rounded-lg">
							{#if user?.avatarUrl}
								<Avatar.Image src={user.avatarUrl} alt={user.username} />
							{/if}
							<Avatar.Fallback class="rounded-lg">
								{userInitials(user?.username ?? '')}
							</Avatar.Fallback>
						</Avatar.Root>
						<span class="flex min-w-0 flex-1 flex-col text-left leading-tight">
							<span class="truncate text-sm font-semibold">{user?.username}</span>
							<span class="text-sidebar-foreground/70 truncate text-xs"
								>{user?.email}</span
							>
						</span>
						<ChevronsUpDown class="ml-auto size-4" />
					</SideBar.MenuButton>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content side="right" align="end" class="w-56">
				<DropdownMenu.Item>
					<a href="/settings" class="flex w-full items-center gap-2">
						<Settings class="size-4" /> Settings
					</a>
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<a href="/" class="flex w-full items-center gap-2">
						<ExternalLink class="size-4" /> Public site
					</a>
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<a href="/about" class="flex w-full items-center gap-2">
						<Info class="size-4" /> About Comhairle
					</a>
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</SideBar.MenuItem>
</SideBar.Menu>
