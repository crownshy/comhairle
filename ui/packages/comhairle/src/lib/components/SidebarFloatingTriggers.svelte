<script lang="ts">
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import { Button } from '$lib/components/ui/button';
	import { PanelLeftOpen, Menu } from 'lucide-svelte';

	const sidebar = useSidebar();
</script>

<!-- Floating expand button: visible on desktop when sidebar is collapsed -->
{#if !sidebar.isMobile && sidebar.state === 'collapsed'}
	<Button
		variant="outline"
		size="icon"
		class="fixed left-3 top-3 z-30 size-8 shadow-md"
		onclick={() => sidebar.toggle()}
	>
		<PanelLeftOpen class="size-4" />
		<span class="sr-only">Expand sidebar</span>
	</Button>
{/if}

<!-- Mobile menu button: always visible on mobile -->
{#if sidebar.isMobile}
	<Button
		variant="outline"
		size="icon"
		class="fixed right-3 top-3 z-30 size-8 shadow-md"
		onclick={() => sidebar.toggle()}
	>
		<Menu class="size-4" />
		<span class="sr-only">Open menu</span>
	</Button>
{/if}
