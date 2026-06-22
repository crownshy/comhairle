<script lang="ts">
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';
	import {
		sidebarWidth,
		MIN_WIDTH,
		COLLAPSE_THRESHOLD,
		EXPAND_WIDTH
	} from '$lib/components/sidebarWidth.svelte.js';

	const sidebar = useSidebar();
	const COLLAPSED_PX = 48;

	let startX = 0;
	let startWidth = 0;
	let moved = 0;

	function onPointerDown(e: PointerEvent) {
		if (sidebar.isMobile) return;
		e.preventDefault();
		startX = e.clientX;
		startWidth = sidebar.state === 'collapsed' ? COLLAPSED_PX : sidebarWidth.width;
		moved = 0;
		sidebarWidth.resizing = true;
		(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
		window.addEventListener('pointermove', onPointerMove);
		window.addEventListener('pointerup', onPointerUp, { once: true });
		window.addEventListener('pointercancel', onPointerUp, { once: true });
	}

	function onPointerMove(e: PointerEvent) {
		const delta = e.clientX - startX;
		moved = Math.max(moved, Math.abs(delta));
		const next = startWidth + delta;
		if (next < COLLAPSE_THRESHOLD) {
			if (sidebar.state === 'expanded') sidebar.setOpen(false);
		} else {
			if (sidebar.state === 'collapsed') sidebar.setOpen(true);
			sidebarWidth.set(Math.max(MIN_WIDTH, next));
		}
	}

	function onPointerUp() {
		window.removeEventListener('pointermove', onPointerMove);
		sidebarWidth.resizing = false;
		if (moved < 4) {
			if (sidebar.state === 'collapsed') {
				if (sidebarWidth.width < EXPAND_WIDTH) sidebarWidth.set(EXPAND_WIDTH);
				sidebar.setOpen(true);
				sidebarWidth.persist();
			} else {
				sidebar.setOpen(false);
			}
		} else {
			sidebarWidth.persist();
		}
	}
</script>

<div
	role="separator"
	aria-orientation="vertical"
	aria-label="Resize sidebar"
	title="Drag to resize, click to toggle"
	onpointerdown={onPointerDown}
	class="group/resize absolute inset-y-0 right-0 z-30 hidden w-2 translate-x-1/2 touch-none select-none md:block"
	style="cursor: ew-resize;"
>
	<span
		class="bg-sidebar-border/0 group-hover/resize:bg-sidebar-border/70 pointer-events-none absolute inset-y-0 left-1/2 w-[2px] -translate-x-1/2 transition-colors"
		class:!bg-sidebar-border={sidebarWidth.resizing}
	></span>
</div>
