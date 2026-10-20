<script lang="ts">
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import { Button } from '$lib/components/ui/button';
	import { PanelLeftOpen, CircleHelp } from 'lucide-svelte';
	import { tourStore } from '$lib/tours/tourStore.svelte';

	const sidebar = useSidebar();
</script>

<!-- Mobile menu button: always visible on mobile -->
{#if sidebar.isMobile}
	<Button
		variant="outline"
		size="icon"
		class="fixed top-3 left-3 z-30 size-8 shadow-md"
		onclick={() => sidebar.toggle()}
	>
		<PanelLeftOpen class="size-4" />
		<span class="sr-only">Open menu</span>
	</Button>
{/if}

<!-- Replays the current page's tour, when it has one. -->
{#if tourStore.active}
	<Button
		variant="outline"
		size="icon"
		class="fixed right-3 bottom-3 z-30 size-8 shadow-md"
		onclick={() => tourStore.replay()}
	>
		<CircleHelp class="size-4" />
		<span class="sr-only">Replay page tour</span>
	</Button>
{/if}
