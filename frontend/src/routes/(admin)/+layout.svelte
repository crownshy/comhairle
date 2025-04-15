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
</script>

<div class="grid h-full w-full grid-cols-[300px_1fr]">
	<AdminNav user={data.user} {conversations} />
	<main class="overflow-y-auto p-12">
		{@render children()}
	</main>
</div>
