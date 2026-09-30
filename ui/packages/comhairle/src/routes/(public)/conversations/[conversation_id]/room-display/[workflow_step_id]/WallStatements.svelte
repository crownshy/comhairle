<!--
	@component Up to three statements with vote bars, shown on the wall in place of the map
	when the facilitator picks a group or the consensus view. More would not fit at this size.
-->
<script lang="ts">
	import type { ReportComment } from '$lib/tools/polis/reportTypes';
	import type { RoomDisplaySource } from './source';
	import { voteBarsFor } from './live/liveVotes';
	import RoomVoteBar from './RoomVoteBar.svelte';

	type Props = {
		title: string;
		statements: ReportComment[];
		source: RoomDisplaySource;
		/** Message shown when there are no statements yet. */
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
