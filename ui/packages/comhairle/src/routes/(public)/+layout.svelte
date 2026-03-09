<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';

	let { children, data }: LayoutProps = $props();
	const isEmbed = $derived(page.url.searchParams.get('embed') === 'true');

	let isAdmin = $derived(
		data.userRoles
			? data.userRoles.find((ur) => ur.resource === 'Site')?.roles.includes('Admin')
			: false
	);
</script>

<div class="flex min-h-screen w-full flex-col">
	{#if !isEmbed}
		<NavBar user={data.user} {isAdmin} />
	{/if}
	<div class="mx-auto min-h-[80vh] w-full max-w-[1300px] grow px-4 md:px-20">
		{@render children()}
	</div>
	{#if !isEmbed}
		<Footer />
	{/if}
</div>
