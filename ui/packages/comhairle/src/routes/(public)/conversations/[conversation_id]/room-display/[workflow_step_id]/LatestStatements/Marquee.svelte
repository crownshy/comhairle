<!--
	@component The newest statements scrolling along the bottom of the wall. Kept as an
	alternative to `Default.svelte` until it is tested in a real room (NOTES.md).
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';

	type Props = {
		/** Newest first. */
		comments: ReportComment[];
		secondsPerStatement?: number;
	};

	let { comments, secondsPerStatement = 18 }: Props = $props();

	// Once capped, new arrivals swap text instead of changing the loop length and speed.
	const MAX_CHIPS = 12;

	const shown = $derived(comments.slice(0, MAX_CHIPS));
	const duration = $derived(Math.max(shown.length, 1) * secondsPerStatement);
</script>

<div class="flex shrink-0 flex-col gap-2">
	<p class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase">
		Latest statements
	</p>

	{#if shown.length === 0}
		<p class="text-muted-foreground py-4 text-xl">
			Statements appear here as the room writes them.
		</p>
	{:else}
		<div class="marquee">
			<div class="marquee-track" style="--marquee-duration: {duration}s">
				{#each shown as comment (comment.tid)}
					<span class="chip">{comment.text}</span>
				{/each}
				<!-- A second copy makes the loop seamless; the track scrolls by exactly half. -->
				{#each shown as comment (comment.tid)}
					<span class="chip" aria-hidden="true">{comment.text}</span>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.marquee {
		overflow: hidden;
		mask-image: linear-gradient(
			to right,
			transparent,
			black 4rem,
			black calc(100% - 4rem),
			transparent
		);
	}

	/* Spacing is a margin on each chip, not `gap`, so one copy is exactly half the track and the loop does not stutter. */
	.marquee-track {
		display: flex;
		width: max-content;
		animation: marquee var(--marquee-duration) linear infinite;
	}

	.chip {
		border: 1px solid var(--border);
		background: var(--card);
		color: var(--card-foreground);
		border-radius: var(--radius);
		padding: 0.75rem 1.5rem;
		margin-right: 1rem;
		font-size: 1.5rem;
		line-height: 2rem;
		white-space: nowrap;
	}

	@keyframes marquee {
		to {
			transform: translateX(-50%);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.marquee-track {
			animation: none;
		}
	}
</style>
