<script lang="ts">
	// PROTOTYPE. Same collapse behaviour as AdminNav, lifted so variants don't each re-implement it.
	import { Button } from '$lib/components/ui/button';
	import { PanelLeftClose, PanelLeftOpen } from 'lucide-svelte';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import { useSidebarWidth } from '$lib/components/sidebarWidthContext.svelte.js';
	import { EXPAND_WIDTH } from '$lib/components/sidebarWidth.js';

	const sidebar = useSidebar();
	const sidebarWidth = useSidebarWidth();

	function expand() {
		if (sidebarWidth.width < EXPAND_WIDTH) sidebarWidth.setWidth(EXPAND_WIDTH);
		sidebar.setOpen(true);
		sidebarWidth.persist();
	}
</script>

<Button
	variant="ghost"
	size="icon"
	class="text-sidebar-foreground/70 size-7 shrink-0 group-data-[collapsible=icon]:hidden"
	onclick={() => sidebar.toggle()}
>
	<PanelLeftClose class="size-4" />
	<span class="sr-only">Collapse sidebar</span>
</Button>
<Button
	variant="ghost"
	size="icon"
	class="text-sidebar-foreground/70 mx-auto hidden size-7 group-data-[collapsible=icon]:flex"
	onclick={expand}
>
	<PanelLeftOpen class="size-4" />
	<span class="sr-only">Expand sidebar</span>
</Button>
