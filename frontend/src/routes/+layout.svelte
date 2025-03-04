<script lang="ts">
	import type { LayoutProps } from './$types';
	import { i18n } from '$lib/i18n';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
	import '../app.css';
	import NavBar from '$lib/components/NavBar.svelte';
	import { notifications, NotificationsToaster } from '$lib/notifications.svelte';
	let { children, data }: LayoutProps = $props();
	
	$effect(() => {
		notifications.listen();
	});

</script>

<ParaglideJS {i18n}>
	<NotificationsToaster closeButton />
	<NavBar user={data.user} />
	<div class="mx-auto mb-16 w-full max-w-[1300px] pt-40 md:px-20">
		{@render children()}
	</div>
</ParaglideJS>
