<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { ProfileMenu, LoginButtons } from '$lib/profile';
	import LocaleSwitcher from '$lib/components/LocaleSwitcher.svelte';
	import * as m from '$lib/paraglide/messages';
	import * as Drawer from '$lib/components/ui/drawer';
	import { HamburgerMenu } from 'svelte-radix';
	import { afterNavigate } from '$app/navigation';
	import UserNavMenu from './UserNavMenu.svelte';

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
</script>

<nav class="bg-primary text-muted-foreground z-10 flex w-full flex-col items-center p-5 shadow-md">
	<div class="margin-auto container flex max-w-[1200px] items-center justify-between">
		<ComhairleLogo logoSize="sm" textClass="text-nav-text invisible hidden lg:visible lg:block" />

		<!-- Desktop Navigation -->
		<div class="hidden space-x-6 md:flex lg:space-x-5">
			{#each links as link (link.href)}
				<Button href={link.href} variant="nav">{link.name}</Button>
			{/each}
		</div>

		<div class="hidden gap-x-5 md:flex">
			<LocaleSwitcher class="rounded-full border-none bg-transparent data-[placeholder]:text-primary-foreground shadow-xs"/>
			<ProfileMenu {user} />
			{#if isAdmin}
				<Button variant="nav" href="/admin">Dashboard</Button>
			{/if}
		</div>

		<!-- Mobile Navigation -->
		<div class="md:hidden">
			<Drawer.Root bind:open={isOpen} direction="bottom">
				<Drawer.Trigger>
					<Button variant="outline">
						<HamburgerMenu />
					</Button>
				</Drawer.Trigger>
				<Drawer.Content>
					<div class="my-auto max-h-[80vh]">
						<div class="h-full p-4">
							<div class="flex h-full flex-col justify-between">
								<div class="mb-10 flex w-full flex-col items-center justify-center gap-2">
									{#each links as link}
										<Button
											href={link.href}
											variant="ghost"
											class="block py-2 text-xl text-gray-700 hover:text-black"
										>
											{link.name}
										</Button>
									{/each}
									<LocaleSwitcher />
								</div>
								<UserNavMenu {user} />
								{#if isAdmin}
									<Button variant="secondary" href="/admin">Dashboard</Button>
								{/if}
							</div>
						</div>
					</div>
				</Drawer.Content>
			</Drawer.Root>
		</div>
	</div>
</nav>
