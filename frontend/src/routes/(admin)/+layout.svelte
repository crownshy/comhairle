<script lang="ts">
	import AdminNav from '$lib/components/AdminNav.svelte';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirect } from '$lib/urls';

	let { children, data }: LayoutProps = $props();
	let conversations = $derived(data.conversations);

	if (!data.user) {
		loginRedirect(page.url.toString(), 'You need to be logged in to access this');
	}

	let theme = {
		radius: '2rem',
		mutted: 'hsla(100, 29%, 10%, 1)'
	};

	let themeCss = Object.entries(theme).reduce((a, b) => (a = a + `--${b[0]} : ${b[1]};`), '');
</script>

<div style={themeCss} class="bg-admin-background grid h-full w-full grid-cols-[350px_1fr]">
	<AdminNav user={data.user} {conversations} />
	<main class="flex flex-col overflow-y-auto p-12">
		{@render children()}
	</main>
</div>
