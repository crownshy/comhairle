<script lang="ts">
	import { Drawer as DrawerPrimitive } from 'vaul-svelte';

	// vaul-svelte leaves focus outside the drawer on open unless autoFocus is set.
	let {
		shouldScaleBackground = true,
		autoFocus = true,
		open = $bindable(false),
		activeSnapPoint = $bindable(null),
		...restProps
	}: DrawerPrimitive.RootProps = $props();
</script>

<DrawerPrimitive.Root
	{shouldScaleBackground}
	{autoFocus}
	bind:open
	bind:activeSnapPoint
	{...restProps}
/>
