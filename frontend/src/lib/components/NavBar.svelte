<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import Logo from '$lib/assets/comhairle_logo.png';
	import { ProfileMenu, LoginButtons } from '$lib/profile';
	import LanguageSelector from '$lib/components/LanguageSelector.svelte';
	import * as m from '$lib/paraglide/messages';
	let links = [
		{
			href: '/',
			name: 'Home'
		},
		{
			href: '/about',
			name: 'About'
		},
		{
			href: '/conversations',
			name: 'Participate'
		},
		{
			href: '/rights',
			name: 'Your Rights'
		}
	];
	let isOpen = $state(false);

	let { user } = $props();
</script>

<nav class="fixed z-10 flex w-full flex-col bg-white p-4 shadow-md">
	<div class="container flex items-center justify-between">
		<div class="align-center flex flex-row items-center gap-4">
			<img src={Logo} alt="Comhairle Logo" />
			<a href="/" class="invisible text-xl font-bold lg:visible">Comhairle</a>
		</div>

		<!-- Desktop Navigation -->
		<div class="hidden space-x-10 md:flex">
			{#each links as link}
				<a href={link.href} class="text-gray-700 hover:text-black">{link.name}</a>
			{/each}
		</div>

		<div class="flex flex-row gap-4">
			<LanguageSelector />
			<div class="hidden md:flex">
				<ProfileMenu {user} />
			</div>

			<!-- Mobile Menu Button -->
			<Button class="md:hidden" on:click={() => (isOpen = !isOpen)}>
				{isOpen ? 'Close' : 'Menu'}
			</Button>
		</div>
	</div>

	<!-- Mobile Navigation -->
	<div class="md:hidden" class:open={isOpen}>
		{#if isOpen}
			<div class="mt-2 flex w-full flex-col space-y-2">
				{#each links as link}
					<a href={link.href} class="block px-4 py-2 text-gray-700 hover:text-black">{link.name}</a>
				{/each}
				{#if user}
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
				{:else}
					<LoginButtons />
				{/if}
			</div>
		{/if}
	</div>
</nav>

<style>
	.open {
		display: block;
	}
</style>
