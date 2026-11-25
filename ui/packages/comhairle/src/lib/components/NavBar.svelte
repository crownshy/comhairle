<script lang="ts">
	import { logOut } from '$lib/utils/logout';
	import Button from '$lib/components/ui/button/button.svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { ProfileMenu } from '$lib/profile';
	import LocaleSwitcher from '$lib/components/LocaleSwitcher.svelte';
	import { m } from '$lib/paraglide/messages';
	import * as Drawer from '$lib/components/ui/drawer';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Badge } from '$lib/components/ui/badge';
	import ModeToggle from '$lib/components/ModeToggle.svelte';
	import Menu from '@lucide/svelte/icons/menu';
	import Home from '@lucide/svelte/icons/house';
	import Info from '@lucide/svelte/icons/info';
	import MessageSquare from '@lucide/svelte/icons/message-square';
	import Shield from '@lucide/svelte/icons/shield';
	import Settings from '@lucide/svelte/icons/settings';
	import Bell from '@lucide/svelte/icons/bell';
	import LogOut from '@lucide/svelte/icons/log-out';
	import Briefcase from '@lucide/svelte/icons/briefcase';
	import LayoutGrid from '@lucide/svelte/icons/layout-grid';
	import CircleHelp from '@lucide/svelte/icons/circle-question-mark';
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { userInitials } from '$lib/utils';
	import { Separator } from '$lib/components/ui/separator';
	import { notificationService } from '$lib/services/notifications.svelte';
	import { useSupportDrawer } from '$lib/components/supportDrawerContext.svelte';
	import { cn } from '$lib/utils';
	import type { UserDto } from '@crownshy/api-client/api';

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
	// The logo already goes home, and the home page is the conversation list, so the bar
	// leaves out Home and Participate. The menu drawer keeps them.
	const barLinks = links.filter((link) => link.href !== '/' && link.href !== '/conversations');
	let isOpen = $state(false);

	// Close the menu on navigate
	afterNavigate(() => {
		isOpen = false;
	});

	type Props = {
		user: UserDto | null | undefined;
		isAdmin: boolean;
		/** The page opens with a light hero, which the bar sits on, see-through, until scrolled. */
		overHero?: boolean;
	};
	let { user, isAdmin, overHero = false }: Props = $props();

	let scrollY = $state(0);
	let scrolled = $derived(scrollY > 8);
	let light = $derived(overHero && !scrolled);

	let user_initials = $derived(userInitials(user?.username ?? ''));

	const supportDrawer = useSupportDrawer();
	let hasSupportDrawer = $derived(page.data.hasSupportDrawer === true);

	const linkIcons = [Home, Info, MessageSquare, Shield];

	function isCurrentLink(href: string) {
		return page.url.pathname === href || (href !== '/' && page.url.pathname.startsWith(href));
	}
</script>

<svelte:window bind:scrollY />

<!-- The bar stays at the top as the page scrolls, slightly see-through once it does.
	Over the light hero the bar's text colour (primary-foreground) becomes the page's dark
	text, and the Sign up pill turns dark, so everything in it reads on the light gradient. -->
<nav
	aria-label={m.nav_main_label()}
	class={cn(
		'text-muted-foreground sticky top-0 z-30 flex w-full flex-col items-center justify-center py-6 transition-[background-color,box-shadow] duration-300',
		light
			? 'bg-transparent [--nav-pill-text:var(--background)] [--primary-foreground:var(--foreground)]'
			: 'shadow-md',
		!light && (scrolled ? 'bg-primary/90 backdrop-blur-xl backdrop-saturate-150' : 'bg-primary')
	)}
>
	<div class="margin-auto container flex max-w-[1280px] items-center justify-between px-6">
		<div class="lg:hidden">
			<ComhairleLogo logoSize="md" showText={false} surface={light ? 'page' : 'dark'} />
		</div>
		<div class="hidden lg:block">
			<ComhairleLogo logoSize="md" surface={light ? 'page' : 'dark'} />
		</div>

		<!-- Desktop: the links sit on the right with the language picker and account buttons. -->
		<div class="navbar:flex ml-auto hidden items-center gap-x-2">
			{#each barLinks as link (link.href)}
				<Button
					href={link.href}
					variant="nav"
					aria-current={isCurrentLink(link.href) ? 'page' : undefined}
					class={cn(
						'h-10 px-4 text-base font-medium shadow-none',
						isCurrentLink(link.href) &&
							(light
								? 'bg-foreground/10'
								: 'bg-sidebar/50 dark:bg-primary-foreground dark:text-primary')
					)}>{link.name}</Button
				>
			{/each}
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
					{m.workspace()}
				</Button>
			{/if}
			<ProfileMenu {user} />
		</div>

		<div class="flex items-center gap-2">
			{#if hasSupportDrawer}
				<Button
					variant="nav"
					class="gap-2 rounded-full text-base font-normal"
					aria-label={m.support_find_out_more()}
					onclick={() => (supportDrawer.open = true)}
				>
					<CircleHelp class="size-5" aria-hidden="true" />
					<span class="max-sm:hidden" aria-hidden="true">{m.support_find_out_more()}</span
					>
				</Button>
			{/if}

			<!-- Mobile Navigation -->
			<div class="navbar:hidden">
				<Drawer.Root bind:open={isOpen} direction="bottom">
					<Drawer.Trigger>
						{#snippet child({ props })}
							<Button {...props} variant="nav" size="icon" aria-label={m.open_menu()}>
								<Menu class="size-7" />
							</Button>
						{/snippet}
					</Drawer.Trigger>
					<Drawer.Content>
						<Drawer.Title class="sr-only">{m.nav_drawer_title()}</Drawer.Title>
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
											{#if user.authType === 'guest'}{m.guest()}{:else}{user.guestCode}{/if}
										</span>
										{#if user.email}
											<span class="text-muted-foreground text-sm"
												>{user.email}</span
											>
										{/if}
									</div>
								</div>
								<Separator />
							{/if}

							<!-- Navigation links -->
							<nav aria-label={m.nav_menu_label()} class="flex flex-col gap-0.5">
								{#each links as link, i (link.href)}
									{@const Icon = linkIcons[i]}
									<Button
										href={link.href}
										variant="ghost"
										aria-current={isCurrentLink(link.href) ? 'page' : undefined}
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
										{#if notificationService.unreadCount > 0}
											<Badge class="ml-auto"
												>{notificationService.unreadCount}</Badge
											>
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
										{m.workspace()}
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
										class="text-muted-foreground text-sm font-medium tracking-wider uppercase"
										>{m.language()}</span
									>
									<LocaleSwitcher />
								</div>
								<ModeToggle />
							</div>

							<!-- Auth actions -->
							{#if user}
								<Separator />
								<form
									method="POST"
									onsubmit={(e) => {
										e.preventDefault();
										logOut();
									}}
									class="px-0"
								>
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
	</div>
</nav>
