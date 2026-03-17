<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { ProfileMenu, LoginButtons } from '$lib/profile';
	import LocaleSwitcher from '$lib/components/LocaleSwitcher.svelte';
	import * as m from '$lib/paraglide/messages';
	import * as Drawer from '$lib/components/ui/drawer';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Badge } from '$lib/components/ui/badge';
	import ModeToggle from '$lib/components/ModeToggle.svelte';
	import {
		Menu,
		Home,
		Info,
		MessageSquare,
		Shield,
		Settings,
		Bell,
		LogOut,
		Briefcase
	} from 'lucide-svelte';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { userInitials } from '$lib/utils';
	import { apiClient } from '@crownshy/api-client/client';
	import { Separator } from '$lib/components/ui/separator';

	let links = [
		{
			href: '/',
			name: m.home()
		},
		{
			href: '/about',
			name: m.about()
		},
		{
			href: '/conversations',
			name: m.participate()
		},
		{
			href: '/rights',
			name: m.your_rights()
		}
	];
	let isOpen = $state(false);

	// Close the menu on navigate
	afterNavigate(() => {
		isOpen = false;
	});

	let { user, isAdmin } = $props();

	let user_initials = $derived(userInitials(user?.username ?? ''));
	let notifications: number | undefined = $state();

	const linkIcons = [Home, Info, MessageSquare, Shield];

	$effect(() => {
		if (!user) return;
		async function checkNotifications() {
			notifications = (await apiClient.GetUnreadNotificationsCount()).count;
		}
		checkNotifications();
		const interval = setInterval(checkNotifications, 5000);
		return () => clearInterval(interval);
	});
</script>

<nav
	class="bg-primary text-muted-foreground z-10 flex w-full flex-col items-center justify-center py-6 shadow-md"
>
	<div class="margin-auto container flex max-w-[1280px] items-center justify-between px-6">
		<div class="lg:hidden">
			<ComhairleLogo logoSize="md" showText={false} />
		</div>
		<div class="hidden lg:block">
			<ComhairleLogo logoSize="md" />
		</div>

		<!-- Desktop Navigation -->
		<div class="hidden gap-3 md:flex">
			{#each links as link (link.href)}
				<Button
					href={link.href}
					variant="nav"
					class="h-10 text-base font-normal {page.url.pathname === link.href ||
					(link.href !== '/' && page.url.pathname.startsWith(link.href))
						? 'bg-sidebar/50 shadow-xs'
						: ''}">{link.name}</Button
				>
			{/each}
		</div>

		<div class="hidden items-center gap-x-4 md:flex">
			<LocaleSwitcher
				class="data-[placeholder]:text-primary-foreground rounded-full border border-none bg-transparent py-5 text-base shadow-xs hover:bg-white/10"
			/>
			{#if isAdmin}
				<Button
					variant="nav"
					href="/admin"
					size="lg"
					class="gap-2 rounded-full text-base font-normal"
				>
					<Briefcase class="size-4" />
					Workspace
				</Button>
			{/if}
			<ProfileMenu {user} />
		</div>

		<!-- Mobile Navigation -->
		<div class="md:hidden">
			<Drawer.Root bind:open={isOpen} direction="bottom">
				<Drawer.Trigger>
					<Button variant="nav" size="icon">
						<Menu class="size-7" />
					</Button>
				</Drawer.Trigger>
				<Drawer.Content>
					<div class="mx-auto flex w-full max-w-md flex-col gap-1 p-4 pb-8">
						<!-- User section -->
						{#if user}
							<div class="flex items-center gap-3 px-3 py-3">
								<Avatar.Root class="h-10 w-10">
									{#if user.avatarUrl}
										<Avatar.Image
											src={user.avatarUrl}
											alt={user.username ?? ''}
										/>
									{/if}
									<Avatar.Fallback class="text-sm"
										>{user_initials}</Avatar.Fallback
									>
								</Avatar.Root>
								<div class="flex flex-col">
									<span class="text-foreground text-sm font-medium">
										{#if user.authType === 'annon'}Anonymous{:else}{user.username}{/if}
									</span>
									{#if user.email}
										<span class="text-muted-foreground text-xs"
											>{user.email}</span
										>
									{/if}
								</div>
							</div>
							<Separator />
						{/if}

						<!-- Navigation links -->
						<nav class="flex flex-col gap-0.5">
							{#each links as link, i (link.href)}
								{@const Icon = linkIcons[i]}
								<Button
									href={link.href}
									variant="ghost"
									class="text-foreground h-11 w-full justify-start gap-3 rounded-lg px-3 text-base font-normal"
								>
									<Icon class="text-muted-foreground size-5" />
									{link.name}
								</Button>
							{/each}
						</nav>

						<Separator />

						<!-- Settings & account -->
						<div class="flex flex-col gap-0.5">
							{#if user}
								<Button
									href="/notifications"
									variant="ghost"
									class="text-foreground h-11 w-full justify-start gap-3 rounded-lg px-3 text-base font-normal"
								>
									<Bell class="text-muted-foreground size-5" />
									{m.notifications()}
									{#if notifications && notifications > 0}
										<Badge class="ml-auto">{notifications}</Badge>
									{/if}
								</Button>
								<Button
									href="/settings"
									variant="ghost"
									class="text-foreground h-11 w-full justify-start gap-3 rounded-lg px-3 text-base font-normal"
								>
									<Settings class="text-muted-foreground size-5" />
									{m.settings()}
								</Button>
							{/if}
							{#if isAdmin}
								<Button
									href="/admin"
									variant="ghost"
									class="text-foreground h-11 w-full justify-start gap-3 rounded-lg px-3 text-base font-normal"
								>
									<LayoutGrid class="text-muted-foreground size-5" />
									Workspace
								</Button>
							{/if}
						</div>

						{#if user}
							<Separator />
						{/if}

						<!-- Preferences -->
						<div class="flex flex-col gap-2 px-3 py-2">
							<div class="flex items-center justify-between">
								<span
									class="text-muted-foreground text-xs font-medium tracking-wider uppercase"
									>{m.language()}</span
								>
								<LocaleSwitcher />
							</div>
							<ModeToggle />
						</div>

						<!-- Auth actions -->
						{#if user}
							<Separator />
							<form method="POST" action="/auth/logout" class="px-0">
								<Button
									type="submit"
									variant="ghost"
									class="text-destructive hover:text-destructive h-11 w-full justify-start gap-3 rounded-lg px-3 text-base font-normal"
								>
									<LogOut class="size-5" />
									{m.logout()}
								</Button>
							</form>
						{:else}
							<Separator />
							<div class="flex gap-2 px-3 pt-2 text-base">
								<Button
									href="/auth/login"
									variant="outline"
									class="flex-1 text-base">{m.login()}</Button
								>
								<Button href="/auth/signup" class="flex-1 text-base"
									>{m.sign_up()}</Button
								>
							</div>
						{/if}
					</div>
				</Drawer.Content>
			</Drawer.Root>
		</div>
	</div>
</nav>
