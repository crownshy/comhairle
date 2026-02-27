<script lang="ts">
	import AdminNav from '$lib/components/AdminNav.svelte';
	import SidebarFloatingTriggers from '$lib/components/SidebarFloatingTriggers.svelte';
	import * as SideBar from '$lib/components/ui/sidebar';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { loginRedirect } from '$lib/urls';

	let { children, data }: LayoutProps = $props();
	let conversations = $derived(data.conversations);

	if (!data.user) {
		loginRedirect(page.url.toString(), 'You need to be logged in to access this');
	}
</script>

<SideBar.Provider>
	<AdminNav user={data.user} conversations={conversations.records} path={page.url.pathname} />
	<SideBar.Inset>
		<SidebarFloatingTriggers />
		<main class="bg-muted flex flex-grow flex-col overflow-y-auto">
			{@render children()}
		</main>
	</SideBar.Inset>
</SideBar.Provider>
