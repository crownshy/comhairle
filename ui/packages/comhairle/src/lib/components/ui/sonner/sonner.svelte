<script lang="ts">
	import { Toaster as Sonner, type ToasterProps as SonnerProps } from 'svelte-sonner';
	import { themeStore } from '$lib/stores/theme.svelte';

	let { ...restProps }: SonnerProps = $props();
</script>

<Sonner
	theme={themeStore.isDark ? 'dark' : 'light'}
	class="toaster group"
	style="--normal-bg: var(--color-popover); --normal-text: var(--color-popover-foreground); --normal-border: var(--color-border); z-index: 40;"
	{...restProps}
/>
