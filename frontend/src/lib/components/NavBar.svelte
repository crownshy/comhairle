<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Logo from '$lib/assets/comhairle_logo.png';
	import { ProfileMenu, LoginButtons } from '$lib/profile';
	import LanguageSelector from '$lib/components/LanguageSelector.svelte';
	import * as m from '$lib/paraglide/messages';
	import * as Drawer from '$lib/components/ui/drawer';
	import { HamburgerMenu } from 'svelte-radix';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import UserAvatar from './UserAvatar.svelte';

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

	let { user } = $props();
</script>

<nav class="z-10 flex w-full flex-col bg-white p-4 shadow-md">
	<div class="container flex items-center justify-between">
		<div class="align-center flex flex-row items-center gap-4">
			<img src={Logo} alt="Comhairle Logo" />
			<a href="/" class=" invisible hidden text-xl font-bold lg:visible lg:block">Comhairle</a>
		</div>

		<!-- Desktop Navigation -->
		<div class="hidden space-x-6 md:flex lg:space-x-10">
			{#each links as link}
				<a href={link.href} class="text-gray-700 hover:text-black">{link.name}</a>
			{/each}
		</div>

		<div class="hidden md:flex">
			<LanguageSelector />
			<ProfileMenu {user} />
		</div>

		<!-- Mobile Navigation -->
		<div class="md:hidden">
			<Drawer.Root bind:open={isOpen} direction="right">
				<Drawer.Trigger asChild let:builder>
					<Button builders={[builder]} variant="outline">
						<HamburgerMenu />
					</Button>
				</Drawer.Trigger>
				<Drawer.Content>
					<div class="my-auto h-screen">
						<div class="h-full p-4">
							<div class="flex h-full flex-col justify-between">
								<div class="flex w-full flex-col items-center justify-center gap-2">
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
								<div class="flex w-full flex-col items-center gap-4">
									{#if user}
										<div class="flex flex-col gap-4">
											<UserAvatar {user} />
											<form method="POST" action="/auth/logout">
												<Button
													type="submit"
													variant="outline"
													fullWidth
													class="text-gray-700 hover:text-black"
												>
													{m.logout()}
												</Button>
											</form>
										</div>
									{:else}
										<LoginButtons />
									{/if}
								</div>
							</div>
						</div>
					</div>
				</Drawer.Content>
			</Drawer.Root>
		</div>
	</div>
</nav>
