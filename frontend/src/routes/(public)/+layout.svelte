<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();
	let isAdmin = $derived(
		data.userRoles
			? data.userRoles.find((ur) => ur.resource === 'Site')?.roles.includes('Admin')
			: false
	);
</script>

<div class="flex w-full flex-col">
	<NavBar user={data.user} {isAdmin} />
	<div class="mx-auto min-h-screen w-full max-w-[1300px] grow px-4 md:px-20">
		{@render children()}
	</div>
	<Footer />
</div>
