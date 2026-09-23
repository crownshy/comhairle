<!--
	@component A handful of statements with vote bars, at room scale, for the wall.
	Takes the map's place when the facilitator picks an opinion group or asks for the
	consensus statements on the console.

	Three at most. Three statements at headline size, each with its bars in one row,
	is what fits on a projector; a longer list would have to shrink to body copy, which
	is the failure the whole display exists to avoid.

	Bars come from the source: folded from the vote stream when it carries real votes,
	read off the report comment when its matrix is apportioned (`voteBarsFor`).
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import type { RoomDisplaySource } from '$lib/room-display/source';
	import { voteBarsFor } from '$lib/room-display/liveVotes';
	import RoomVoteBar from './RoomVoteBar.svelte';

	type Props = {
		title: string;
		statements: ReportComment[];
		source: RoomDisplaySource;
		/** What to say when there is nothing to show yet. */
		empty: string;
	};

	let { title, statements, source, empty }: Props = $props();

	const shown = $derived(statements.slice(0, 3));
</script>

<div class="flex h-full min-h-0 flex-col gap-6">
	<p
		class="text-muted-foreground shrink-0 text-xl font-medium tracking-wide uppercase lg:text-2xl"
	>
		{title}
	</p>
	{#if shown.length === 0}
		<p class="text-muted-foreground text-2xl lg:text-3xl">{empty}</p>
	{:else}
		<ol class="flex min-h-0 flex-1 flex-col justify-around gap-6 overflow-hidden">
			{#each shown as statement (statement.tid)}
				{@const bars = voteBarsFor(source, statement)}
				<li class="fade-in flex flex-col gap-3">
					<p
						class="text-foreground text-2xl leading-snug font-semibold text-balance lg:text-3xl"
					>
						{statement.text}
					</p>
					<!-- One column per bar, so a statement is two lines tall however many groups there are. -->
					<div
						class="grid max-w-5xl gap-8"
						style="grid-template-columns: repeat({1 +
							bars.groups.length}, minmax(0, 1fr));"
					>
						<RoomVoteBar {...bars.overall} />
						{#each bars.groups as bar (bar.label)}
							<RoomVoteBar {...bar} />
						{/each}
					</div>
				</li>
			{/each}
		</ol>
	{/if}
</div>

<style>
	.fade-in {
		animation: fade-in 400ms ease both;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(6px);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.fade-in {
			animation: none;
		}
	}
</style>
