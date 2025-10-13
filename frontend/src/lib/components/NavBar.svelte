<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Logo from '$lib/assets/comhairle_logo.svg';
	import { ProfileMenu, LoginButtons } from '$lib/profile';
	import LanguageSelector from '$lib/components/LanguageSelector.svelte';
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
		<div class="align-center flex flex-row items-center gap-4">
			<img src={Logo} alt="Comhairle Logo" />
			<a href="/" class=" text-nav-text invisible hidden text-xl font-bold lg:visible lg:block"
				>Comhairle</a
			>
		</div>

		<!-- Desktop Navigation -->
		<div class="hidden space-x-6 md:flex lg:space-x-5">
			{#each links as link}
				<Button href={link.href} variant="nav">{link.name}</Button>
			{/each}
		</div>

		<div class="hidden gap-x-5 md:flex">
			<LanguageSelector />
			<ProfileMenu {user} />
			{#if isAdmin}
				<Button variant="default" href="/admin">Admin</Button>
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
									<LanguageSelector />
								</div>
								<UserNavMenu {user} />
								{#if isAdmin}
									<Button variant="secondary" href="/admin">Admin</Button>
								{/if}
							</div>
						</div>
					</div>
				</Drawer.Content>
			</Drawer.Root>
		</div>
	</div>
</nav>
