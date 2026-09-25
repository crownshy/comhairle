<!--
	@component The newest statements, still. The default presentation of the `marquee`
	block; `LatestStatementsMarquee.svelte` is the scrolling alternative.

	Why this exists: a continuously scrolling ticker cannot be read from across a room.
	The eye has to acquire a moving target and then track it, and at eight metres it
	loses before the sentence is finished. Slowing the scroll does not fix it, because
	the problem is the motion rather than the speed; it just means fewer statements go
	past while still being unreadable.

	So the motion happens on arrival and nowhere else. A slot opens at the head, the
	rest slide along as it grows, the new statement fades into it, the oldest drops
	off, and then everything is completely still until the next one. That is exactly
	the accent CONTEXT.md describes: in-place, non-blocking, never taking the screen.
	Between arrivals the room is reading type that is not moving.

	The slot opens by layout (a grid row or a flex share growing from nothing) rather
	than by sliding the neighbours with a transform. A transform slide draws the new
	statement on top of the old head for the whole slide, and from across a room that
	reads as two statements printed over each other. Layout cannot overlap.

	Older statements fade out along the list. Recency is the only thing this block is
	saying, and an opacity ramp says it without any motion at all.
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';

	type Props = {
		/** Published statements, newest first. */
		comments: ReportComment[];
		/**
		 * `row` for the wide, short slot along the bottom of a wall; `column` when the
		 * block has height to spend and the list should read top to bottom.
		 */
		direction?: 'row' | 'column';
		/**
		 * How many to show. Defaults to what fits the block's own slot along the bottom
		 * of a wall; the column beside the statement strip is shorter and passes its own.
		 */
		max?: number;
		/**
		 * Less chrome for a wall that is short of height. The block keeps every statement
		 * it would otherwise show: what gives is padding and type size, because a
		 * statement dropped is a person unheard.
		 */
		compact?: boolean;
	};

	let { comments, direction = 'row', max, compact = false }: Props = $props();

	// A row runs out of width before it runs out of statements, and a column out of
	// height. Both are small: this block is "what was just said", not a transcript.
	const limit = $derived(max ?? (direction === 'row' ? 4 : 3));
	const shown = $derived(comments.slice(0, limit));

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
		<ol
			class="flex min-w-0 gap-4 {direction === 'row' ? 'flex-row items-stretch' : 'flex-col'}"
		>
			{#each shown as comment, index (comment.tid)}
				<!--
					The slot: a grid whose one row (column) or flex share (row) grows from
					nothing on arrival. The box inside clips while the slot is opening and
					fades in once it has opened, so no reflowing text is ever seen.
				-->
				<li
					class="slot fade min-w-0 {direction === 'row'
						? 'slot-row flex-1'
						: 'slot-column'}"
					style="opacity: {fade(index, shown.length)}"
				>
					<div class="min-h-0 min-w-0 overflow-hidden">
						<div
							class="arrive border-border bg-card text-card-foreground h-full rounded-lg border leading-snug text-balance {compact
								? 'px-4 py-2 text-base lg:text-lg'
								: 'px-5 py-3 text-xl lg:text-2xl'}"
						>
							{comment.text}
						</div>
					</div>
				</li>
			{/each}
		</ol>
	{/if}
</div>

<style>
	.slot {
		display: grid;
	}

	.slot-column {
		grid-template-rows: 1fr;
		animation: open-column 450ms ease backwards;
	}

	.slot-row {
		animation: open-row 450ms ease backwards;
	}

	/*
		Delayed by the slot's opening, and `backwards` so the delay is spent invisible.
		`backwards` rather than `both` for the fill because a forwards fill would hold
		the value the animation ended on, and this element is repainted as statements
		shift along.
	*/
	.arrive {
		animation: arrive 300ms ease 450ms backwards;
	}

	/* On the slot, because that is where the inline opacity lives. */
	.fade {
		transition: opacity 450ms ease;
	}

	@keyframes open-column {
		from {
			grid-template-rows: 0fr;
		}
	}

	@keyframes open-row {
		from {
			flex-grow: 0;
		}
	}

	@keyframes arrive {
		from {
			opacity: 0;
			transform: translateY(-0.5rem);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.slot-column,
		.slot-row,
		.arrive {
			animation: none;
		}

		.fade {
			transition: none;
		}
	}
</style>
