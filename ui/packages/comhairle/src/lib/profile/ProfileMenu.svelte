<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/form';
	import { buttonVariants } from '$lib/components/ui/button';
	import LoginButtons from './LoginButtons.svelte';
	import { userInitials } from '$lib/utils';
	import { apiClient } from '@crownshy/api-client/client';
	import { Badge } from '$lib/components/ui/badge';
	import { Bell, LogOut, Settings, ChevronsUpDown } from 'lucide-svelte';
	import ModeToggle from '$lib/components/ModeToggle.svelte';
	import type { UserDto } from '@crownshy/api-client/api';

	type Props = {
		user: UserDto;
		triggerVariant?: 'outline' | 'nav';
	};
	const { user, triggerVariant = 'outline' }: Props = $props();

	let user_initials = $derived(userInitials(user?.username ?? ''));
	let notifications: number | undefined = $state();

	$effect(() => {
		async function checkNotifications() {
			notifications = (await apiClient.GetUnreadNotificationsCount()).count;
		}
	});
</script>

{#if user}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class={buttonVariants({ variant: triggerVariant })}>
			<Avatar.Root class="mr-4 h-6 w-6">
				{#if user.avatarUrl}
					<Avatar.Image src={user.avatarUrl} alt="@shadcn" />
				{/if}
				<Avatar.Fallback class="text-foreground">{user_initials}</Avatar.Fallback>
			</Avatar.Root>
			<p class="text-foreground">
				{#if user.authType === 'annon'}
					Anonymous
				{:else}
					{user.username}
				{/if}
			</p>
			{#if notifications && notifications > 0}
				<Badge>{notifications}</Badge>
			{/if}
			<ChevronsUpDown class="size-3" />
		</DropdownMenu.Trigger>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
				<DropdownMenu.Item>
					{#if user.authType === 'annon'}
						<h2>Your ID: {user.username}</h2>
					{/if}
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<Button href="/settings" type="submit" variant="ghost"
						><Settings />Settings</Button
					>
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<ModeToggle />
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<Button href="/notifications" type="submit" variant="ghost"
						><Bell />Notifications
						{#if notifications && notifications > 0}
							<Badge>{notifications}</Badge>
						{/if}
					</Button>
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					<form method="POST" action="/auth/logout">
						<Button type="submit" variant="ghost"><LogOut />Logout</Button>
					</form>
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{:else}
	<LoginButtons />
{/if}
