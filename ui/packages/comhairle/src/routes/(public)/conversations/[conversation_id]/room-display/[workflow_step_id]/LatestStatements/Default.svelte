<!--
	@component The newest statements, held still between arrivals so they can be read
	from across a room. See NOTES.md, "Latest statements: marquee or still?".
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';

	type Props = {
		/** Newest first. */
		comments: ReportComment[];
		direction?: 'row' | 'column';
		max?: number;
		/** Shrinks padding and type on a short wall, but never drops a statement. */
		compact?: boolean;
	};

	let { comments, direction = 'row', max, compact = false }: Props = $props();

	const limit = $derived(max ?? (direction === 'row' ? 4 : 3));
	const shown = $derived(comments.slice(0, limit));

	/** Oldest is faintest, but never fully transparent. */
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
					The slot grows open by layout rather than a transform slide, which would draw
					the new statement over the old one while it moves. Text fades in once it is open.
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

	/* Waits for the slot to open. `backwards` rather than `both`, so the end value is not held while statements shift. */
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
