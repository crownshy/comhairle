<!--
	@component The newest statements, scrolling along the bottom of the wall. The
	alternative presentation of the `marquee` block; `LatestStatements.svelte` is the
	still one, and the default.

	Kept rather than deleted because the claim against it ("moving text cannot be read
	at room distance") is worth testing in a room rather than settling in a review. The
	scroll is deliberately slow now: slow enough that a sentence is on screen long
	enough to finish, which is the most generous version of the idea. If it still loses
	to the still list in front of an actual audience, delete this file.

	Two copies of the track ride a single `translateX(-50%)` loop, so the seam between
	the last chip and the first is invisible. The list is capped: once the room is past
	a dozen statements the track stops growing, which means the loop duration stops
	changing and a new arrival only swaps chip text rather than shifting the speed
	under the room. Below the cap a new statement does restart the loop, which is
	visible but only happens in the first few minutes.
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';

	type Props = {
		/** Published statements, newest first. */
		comments: ReportComment[];
		/** How long one statement takes to cross the wall. */
		secondsPerStatement?: number;
	};

	let { comments, secondsPerStatement = 18 }: Props = $props();

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
				<!-- The second copy is decoration: it exists to hide the seam. -->
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

	/*
		The gap rides on each chip rather than on the track. With `gap` the two copies
		are separated by one extra gap, so a -50% shift lands half a gap short of the
		seam and the loop visibly stutters once per lap. Folding the gap into the chip
		makes one copy exactly half the track.
	*/
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
