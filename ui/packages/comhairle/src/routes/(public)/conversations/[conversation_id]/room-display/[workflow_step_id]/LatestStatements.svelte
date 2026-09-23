<!--
	@component The newest statements, still. The default presentation of the `marquee`
	block; `LatestStatementsMarquee.svelte` is the scrolling alternative.

	Why this exists: a continuously scrolling ticker cannot be read from across a room.
	The eye has to acquire a moving target and then track it, and at eight metres it
	loses before the sentence is finished. Slowing the scroll does not fix it, because
	the problem is the motion rather than the speed; it just means fewer statements go
	past while still being unreadable.

	So the motion happens on arrival and nowhere else. A new statement animates in at
	the head, the rest slide along to make room, the oldest drops off, and then
	everything is completely still until the next one. That is exactly the accent
	CONTEXT.md describes: in-place, non-blocking, never taking the screen. Between
	arrivals the room is reading type that is not moving.

	Older statements fade out along the list. Recency is the only thing this block is
	saying, and an opacity ramp says it without any motion at all.
-->
<script lang="ts">
	import { flip } from 'svelte/animate';
	import type { ReportComment } from '$lib/tools/polis/reportTypes';

	type Props = {
		/** Published statements, newest first. */
		comments: ReportComment[];
		/**
		 * `row` for the wide, short slot along the bottom of a wall; `column` when the
		 * block has height to spend and the list should read top to bottom.
		 */
		direction?: 'row' | 'column';
	};

	let { comments, direction = 'row' }: Props = $props();

	// A row runs out of width before it runs out of statements, and a column out of
	// height. Both are small: this block is "what was just said", not a transcript.
	const shown = $derived(comments.slice(0, direction === 'row' ? 4 : 3));

	/** Oldest is faintest. Never fully transparent: it is still a statement someone wrote. */
	function fade(index: number, total: number): number {
		if (total <= 1) return 1;
		return 1 - (index / (total - 1)) * 0.55;
	}
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
		<!--
			Keyed on the direction so switching it rebuilds the list. Without this, flip
			animates every item from its old row position to its new column one, which is
			a lot of flying text to say "you changed a setting".
		-->
		{#key direction}
			<ol
				class="flex min-w-0 gap-4 {direction === 'row'
					? 'flex-row items-stretch'
					: 'flex-col'}"
			>
				{#each shown as comment, index (comment.tid)}
					<!--
						`animate:flip` owns this element's `animation` property: Svelte drives
						the reflow with a generated keyframe and restores what was there when
						it finishes. A CSS `animation` of our own on the same element fights
						it for that property and leaves a stuck transform behind, so the
						arrival animation goes on the box inside.
					-->
					<li
						animate:flip={{ duration: 450 }}
						class="fade min-w-0 {direction === 'row' ? 'flex-1' : ''}"
						style="opacity: {fade(index, shown.length)}"
					>
						<div
							class="arrive border-border bg-card text-card-foreground h-full rounded-lg border px-5 py-3 text-xl leading-snug text-balance lg:text-2xl"
						>
							{comment.text}
						</div>
					</li>
				{/each}
			</ol>
		{/key}
	{/if}
</div>

<style>
	/*
		`backwards` rather than `both`: a forwards fill would hold the value the
		animation ended on, and this element is repainted as statements shift along.
	*/
	.arrive {
		animation: arrive 450ms ease backwards;
	}

	/* On the flipped element, because that is where the inline opacity lives. */
	.fade {
		transition: opacity 450ms ease;
	}

	@keyframes arrive {
		from {
			opacity: 0;
			transform: translateY(-0.5rem);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.arrive {
			animation: none;
		}

		.fade {
			transition: none;
		}
	}
</style>
