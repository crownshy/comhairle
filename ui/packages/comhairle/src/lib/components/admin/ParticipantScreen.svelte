<script lang="ts">
	import type { Snippet } from 'svelte';
	import { DEVICE_SIZES, type Device } from './participantView';

	let {
		device = 'phone',
		scale = 1,
		children
	}: {
		device?: Device;
		scale?: number;
		children: Snippet;
	} = $props();

	let size = $derived(DEVICE_SIZES[device]);
</script>

<!--
	One participant screen, frozen. `inert` is the guarantee, not the styling: it takes
	everything inside out of the tab order, blocks clicks, focus and form submission, and
	drops the subtree from the accessibility tree. Every write in every tool sits behind a
	user action, so blocking input is what makes a participant view safe to mount against
	real config (ADR-0030).

	The outer box carries the scaled dimensions because `transform` does not affect layout,
	so without it a scaled screen would still reserve its full size and leave a gap.
-->
<div
	class="shrink-0"
	style="width:{Math.round(size.width * scale)}px;height:{Math.round(size.height * scale)}px"
>
	<div
		inert
		class="bg-background pointer-events-none origin-top-left overflow-hidden border shadow-sm {device ===
		'phone'
			? 'rounded-[28px]'
			: 'rounded-lg'}"
		style="width:{size.width}px;height:{size.height}px;transform:scale({scale})"
	>
		{@render children()}
	</div>
</div>
