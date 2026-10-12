<script lang="ts">
	import { navigating } from '$app/state';
	import { routeProgress } from '$lib/stores/routeProgress.svelte';

	/**
	 * A page load can be slow enough that a tap on "Join the conversation" looks
	 * like nothing happened. This bar appears on the same frame as the click, for
	 * both client-side navigation and the API calls that precede it.
	 */
	let active = $derived(navigating.to !== null || routeProgress.busy);
</script>

{#if active}
	<div
		class="pointer-events-none fixed inset-x-0 top-0 z-[100] h-1 overflow-hidden"
		role="progressbar"
		aria-label="Loading"
	>
		<div class="bg-primary sweep h-full w-2/5 rounded-full"></div>
	</div>
{/if}

<style>
	.sweep {
		animation: sweep 1.1s ease-in-out infinite;
	}

	@keyframes sweep {
		from {
			transform: translateX(-100%);
		}
		to {
			transform: translateX(350%);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.sweep {
			width: 100%;
			animation: none;
		}
	}
</style>
