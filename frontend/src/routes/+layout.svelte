<script lang="ts">
	import { i18n } from '$lib/i18n';
	import { ParaglideJS } from '@inlang/paraglide-sveltekit';
	import '../app.css';
	import NavBar from '$lib/components/NavBar.svelte';
	import { notifications, NotificationsToaster } from '$lib/notifications.svelte';
	let { children } = $props();

	$effect(() => {
		notifications.listen();
	});
</script>

<ParaglideJS {i18n}>
	<NotificationsToaster closeButton />
	<NavBar />
	<div class="mx-auto mb-16 w-full max-w-[1300px] px-20 pt-40">
		{@render children()}
	</div>
</ParaglideJS>
