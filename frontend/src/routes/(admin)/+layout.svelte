<script lang="ts">
	import AdminNav from '$lib/components/AdminNav.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirect } from '$lib/urls';

	let { children, data }: LayoutProps = $props();
	let conversations = $derived(data.conversations);

	if (!data.user) {
		loginRedirect(page.url.toString(), 'You need to be logged in to access this');
	}

	let theme = {
		radius: '0.5rem',
		mutted: 'hsla(100, 29%, 10%, 1)'
	};

	let themeCss = Object.entries(theme).reduce((a, b) => (a = a + `--${b[0]} : ${b[1]};`), '');
</script>

<div style={themeCss}>
	<SideBar.Provider>
		<AdminNav user={data.user} {conversations} path={page.url.pathname} />
		<main class="flex flex-grow flex-col overflow-y-auto">
			{@render children()}
		</main>
	</SideBar.Provider>
</div>
