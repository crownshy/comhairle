<script lang="ts">
	import { Toaster as Sonner, type ToasterProps as SonnerProps } from 'svelte-sonner';
	import CircleCheck from '@lucide/svelte/icons/circle-check';
	import CircleAlert from '@lucide/svelte/icons/circle-alert';
	import Info from '@lucide/svelte/icons/info';
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
	import X from '@lucide/svelte/icons/x';
	import { themeStore } from '$lib/stores/theme.svelte';

	let { ...restProps }: SonnerProps = $props();
</script>

<!-- svelte-sonner's built-in icons are bare SVGs that screen readers announce as "image".
     The toast text already says what happened, so ours are hidden. -->
<Sonner
	theme={themeStore.isDark ? 'dark' : 'light'}
	class="toaster group"
	style="--normal-bg: var(--color-popover); --normal-text: var(--color-popover-foreground); --normal-border: var(--color-border); z-index: 40;"
	{...restProps}
>
	{#snippet successIcon()}
		<CircleCheck class="size-5" aria-hidden="true" />
	{/snippet}
	{#snippet errorIcon()}
		<CircleAlert class="size-5" aria-hidden="true" />
	{/snippet}
	{#snippet infoIcon()}
		<Info class="size-5" aria-hidden="true" />
	{/snippet}
	{#snippet warningIcon()}
		<TriangleAlert class="size-5" aria-hidden="true" />
	{/snippet}
	{#snippet closeIcon()}
		<X class="size-3" aria-hidden="true" />
	{/snippet}
</Sonner>
