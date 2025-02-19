<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import { onMount } from 'svelte';
	import Logo from '$lib/assets/comhairle_logo.png';
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
	let isOpen = false;
</script>

<nav class="fixed z-10 w-full bg-white p-4 shadow-md">
	<div class="container flex items-center justify-between">
		<div class="align-center flex flex-row items-center">
			<img src={Logo} alt="Comhairle Logo" />
			<a href="/" class="text-xl font-bold">Comhairle</a>
		</div>

		<!-- Desktop Navigation -->
		<div class="hidden space-x-10 md:flex">
			{#each links as link}
				<a href={link.href} class="text-gray-700 hover:text-black">{link.name}</a>
			{/each}
		</div>

		<div class="hidden md:flex">
			<Button>Signup / Login</Button>
		</div>

		<!-- Mobile Menu Button -->
		<Button class="md:hidden" on:click={() => (isOpen = !isOpen)}>
			{isOpen ? 'Close' : 'Menu'}
		</Button>
	</div>

	<!-- Mobile Navigation -->
	<div class="md:hidden" class:open={isOpen}>
		{#if isOpen}
			<div class="mt-2 flex w-full flex-col space-y-2">
				{#each links as link}
					<a href={link.href} class="block px-4 py-2 text-gray-700 hover:text-black">{link.name}</a>
				{/each}
				<Button>Signup / Login</Button>
			</div>
		{/if}
	</div>
</nav>

<style>
	.open {
		display: block;
	}
</style>
