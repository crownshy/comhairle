<!--
	@component PROTOTYPE variant: Stage and console, two surfaces.

	The argument this variant makes: the board is busy because one screen is being asked
	to do two jobs. The room needs a wall it can read from eight metres with no pointer;
	the facilitator needs a dense panel they can scan and click from half a metre. Those
	are opposite requirements and merging them gives you something that is neither.

	So: the projector gets the map and one statement, and everything else (the full
	statement list, the continuum, the counts, the controls) moves to the laptop. The
	statement list becomes clickable here, which it is explicitly not allowed to be on
	the wall (CONTEXT.md, "Statement ticker"): the reason it stays inert there is that
	the room must have one origin of focus, and that reason does not apply to a panel
	only the facilitator sees.

	Shown side by side so the pair can be judged in one screenshot. Two real windows
	would need the two surfaces to share focus state, which is a build question, not a
	design one.
-->
<script lang="ts">
	import type { RoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import type { Scenario } from '$lib/room-display/types';
	import { participantCount } from '$lib/room-display/scenario';
	import { nextUnlock, describeUnlock } from '$lib/room-display/revealStage';
	import OpinionMap from '$lib/room-display/OpinionMap.svelte';
	import StatementStrip from './StatementStrip.svelte';

	type Props = {
		driver: RoomDisplayDriver;
		scenario: Scenario;
		question: string;
	};

	let { driver, scenario, question }: Props = $props();

	let focusedTid = $state<number | null>(null);

	const groupIds = $derived(scenario.groups.map((g) => g.group_id));
	const focused = $derived(driver.state.published.find((c) => c.tid === focusedTid) ?? null);
	const unlock = $derived(nextUnlock(driver.state, driver.stage));
</script>

<div class="grid h-full min-h-0 gap-6 lg:grid-cols-[1.6fr_1fr]">
	<!-- Projector -->
	<section class="border-border flex min-h-0 flex-col gap-4 rounded-lg border p-6">
		<p class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase">
			On the wall
		</p>
		<h1 class="text-foreground shrink-0 text-2xl leading-tight font-bold text-balance">
			{question}
		</h1>
		<div class="min-h-0 flex-1">
			<OpinionMap
				nodes={driver.state.nodes}
				votesByTid={driver.state.votesByTid}
				{focusedTid}
				{groupIds}
			/>
		</div>
		{#if focused}
			{#key focused.tid}
				<p
					class="text-foreground fade-in flex h-24 shrink-0 items-center text-2xl leading-snug font-medium text-balance lg:text-3xl"
				>
					{focused.text}
				</p>
			{/key}
		{:else}
			<p class="text-muted-foreground flex h-24 shrink-0 items-center text-xl">
				Pick a statement on the console to colour the room by it.
			</p>
		{/if}
	</section>

	<!-- Console -->
	<section class="bg-muted flex min-h-0 flex-col gap-4 rounded-lg p-5">
		<p class="text-muted-foreground shrink-0 text-base font-medium tracking-wide uppercase">
			On your laptop
		</p>

		<dl class="grid shrink-0 grid-cols-3 gap-3">
			<div>
				<dt class="text-muted-foreground text-base">Here</dt>
				<dd class="text-foreground text-2xl font-bold tabular-nums">
					{participantCount(driver.state)}
				</dd>
			</div>
			<div>
				<dt class="text-muted-foreground text-base">Votes</dt>
				<dd class="text-foreground text-2xl font-bold tabular-nums">
					{driver.state.totalVotes}
				</dd>
			</div>
			<div>
				<dt class="text-muted-foreground text-base">Statements</dt>
				<dd class="text-foreground text-2xl font-bold tabular-nums">
					{driver.state.published.length}
				</dd>
			</div>
		</dl>

		{#if unlock}
			<p class="text-muted-foreground shrink-0 text-base">
				{describeUnlock(unlock)} until {unlock.stage}
			</p>
		{/if}

		<div class="shrink-0">
			<StatementStrip
				comments={driver.state.published}
				{focusedTid}
				interactive
				height={80}
				onfocusstatement={(tid) => (focusedTid = tid)}
			/>
		</div>

		<!-- Clickable here, inert on the wall. -->
		<ul class="flex min-h-0 flex-col gap-1.5 overflow-y-auto">
			{#each driver.state.published.slice(0, 20) as statement (statement.tid)}
				<li>
					<button
						type="button"
						class="hover:bg-background w-full rounded-md p-2.5 text-left text-base transition-colors"
						class:bg-background={statement.tid === focusedTid}
						class:font-medium={statement.tid === focusedTid}
						onclick={() => (focusedTid = statement.tid)}
					>
						{statement.text}
					</button>
				</li>
			{/each}
		</ul>
	</section>
</div>

<style>
	.fade-in {
		animation: fade-in 400ms ease both;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.fade-in {
			animation: none;
		}
	}
</style>
