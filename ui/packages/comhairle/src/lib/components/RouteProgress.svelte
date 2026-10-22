<script lang="ts">
	import { navigating } from '$app/state';
	import { pendingWork } from '$lib/stores/pendingWork.svelte';

	// A slow page load can make a tap on "Join the conversation" look like it did nothing, so
	// the bar mounts on the click. It stays invisible for the first 100ms so quick loads
	// don't flash it and feel slower.
	let active = $derived(navigating.to !== null || pendingWork.busy);
</script>

{#if active}
	<div
		class="animate-progress-reveal pointer-events-none fixed inset-x-0 top-0 z-[100] h-1 overflow-hidden"
		role="progressbar"
		aria-label="Loading"
	>
		<div
			class="bg-primary animate-progress-sweep h-full w-2/5 rounded-full motion-reduce:w-full motion-reduce:animate-none"
		></div>
	</div>
{/if}
