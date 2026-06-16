<script lang="ts">
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import { Button } from '$lib/components/ui/button';
	import { PanelLeftOpen } from 'lucide-svelte';

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
