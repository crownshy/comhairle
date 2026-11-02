<script lang="ts">
	import { PanelLeftOpen } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';

	type Props = {
		/** The open conversation's name, or the app name outside a conversation. */
		title: string;
	};

	let { title }: Props = $props();

	const sidebar = useSidebar();
</script>

<!-- Below md the sidebar is a drawer, so this bar is the only way to open it. Hidden with CSS
	 rather than `sidebar.isMobile` so it renders on the server too. -->
<header
	class="border-border bg-background flex h-14 w-full shrink-0 items-center gap-3 border-b px-3 md:hidden"
>
	<Button variant="ghost" size="icon" class="size-9 shrink-0" onclick={() => sidebar.toggle()}>
		<PanelLeftOpen class="size-5" />
		<span class="sr-only">Open menu</span>
	</Button>
	<p class="text-foreground truncate text-base font-semibold" {title}>{title}</p>
</header>
