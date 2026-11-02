<script lang="ts">
	import { CircleHelp } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { tourStore } from './tourStore.svelte';
</script>

<!-- Replays the current page's tour, when it has one. -->
{#if tourStore.active}
	<Button
		variant="outline"
		size="icon"
		class="size-8 shadow-md"
		onclick={() => tourStore.replay()}
	>
		<CircleHelp class="size-4" />
		<span class="sr-only">Replay page tour</span>
	</Button>
{/if}
