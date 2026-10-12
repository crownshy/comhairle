<script lang="ts">
	import { resolve } from '$app/paths';
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';
	import { ACTION_KINDS, formatActionDay } from './actionKinds';
	import ActionLogItem from './ActionLogItem.svelte';
	import type { ActivityLogEntry } from './sampleMonitorData';

	type Props = {
		entries: ActivityLogEntry[];
		launchDate: Date;
		closingDate: Date;
		today: Date;
		/** For the "Add activity" link to Recruit; the log is fed from Recruit and Notify. */
		conversationId: string;
	};

	let { entries, launchDate, closingDate, today, conversationId }: Props = $props();

	const WEEKDAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
	const DAY_MS = 24 * 60 * 60 * 1000;

	// Dates are compared as UTC calendar days ("2026-10-12") so time zones can't shift them.
	function dayKey(date: Date): string {
		return date.toISOString().slice(0, 10);
	}

	let byDay = $derived.by(() => {
		const days: Record<string, ActivityLogEntry[]> = {};
		for (const entry of entries) {
			const key = dayKey(entry.date);
			days[key] = [...(days[key] ?? []), entry];
		}
		return days;
	});

	/** Monday on or before launch, through the Sunday on or after closing. */
	let weeks = $derived.by(() => {
		const start = launchDate.getTime() - ((launchDate.getUTCDay() + 6) % 7) * DAY_MS;
		const end = closingDate.getTime() + ((7 - closingDate.getUTCDay()) % 7) * DAY_MS;
		const rows: Date[][] = [];
		for (let time = start; time <= end; time += 7 * DAY_MS) {
			rows.push(Array.from({ length: 7 }, (_, offset) => new Date(time + offset * DAY_MS)));
		}
		return rows;
	});

	let selectedKey = $state<string | null>(null);
	let shownEntries = $derived(
		selectedKey
			? (byDay[selectedKey] ?? [])
			: [...entries].sort((a, b) => a.date.getTime() - b.date.getTime())
	);

	const todayKey = $derived(dayKey(today));
	const launchKey = $derived(dayKey(launchDate));
	const closeKey = $derived(dayKey(closingDate));

	/** Month of the row's Monday, shown only when it differs from the row above. */
	function monthLabel(weekIndex: number): string | null {
		const month = weeks[weekIndex][0].getUTCMonth();
		if (weekIndex > 0 && weeks[weekIndex - 1][0].getUTCMonth() === month) return null;
		return weeks[weekIndex][0].toLocaleDateString('en-GB', { month: 'short', timeZone: 'UTC' });
	}
</script>

<!--
	Weeks from launch to close with each logged action on its day (left) and the log (right).
	Selecting a day filters the log to that day.
-->
<Card.Root>
	<Card.Header>
		<Card.Title>Action calendar</Card.Title>
		<Card.Description class="text-base">
			What the team did and when, from launch to close. Compare it with the signups to see
			what worked.
		</Card.Description>
	</Card.Header>
	<Card.Content class="grid grid-cols-1 gap-6 lg:grid-cols-[minmax(0,1.1fr)_minmax(0,1fr)]">
		<div class="flex flex-col gap-2">
			<div class="grid grid-cols-[2.5rem_repeat(7,minmax(0,1fr))] gap-1 text-center text-sm">
				<span></span>
				{#each WEEKDAYS as weekday (weekday)}
					<span class="text-muted-foreground">{weekday}</span>
				{/each}
			</div>
			{#each weeks as week, weekIndex (dayKey(week[0]))}
				{@const month = monthLabel(weekIndex)}
				<div class="grid grid-cols-[2.5rem_repeat(7,minmax(0,1fr))] gap-1">
					<span class="text-muted-foreground self-center text-sm">{month ?? ''}</span>
					{#each week as day (dayKey(day))}
						{@const key = dayKey(day)}
						{@const dayEntries = byDay[key] ?? []}
						{@const inWindow = key >= launchKey && key <= closeKey}
						{@const isToday = key === todayKey}
						<button
							type="button"
							disabled={dayEntries.length === 0}
							aria-pressed={selectedKey === key}
							aria-label="{formatActionDay(day)}{dayEntries.length
								? `: ${dayEntries.map((entry) => entry.title).join(', ')}`
								: ''}"
							onclick={() => (selectedKey = selectedKey === key ? null : key)}
							class={cn(
								'flex aspect-square min-h-11 flex-col items-center justify-center gap-1 rounded-lg text-sm tabular-nums transition-colors disabled:cursor-default',
								inWindow ? 'bg-muted/50' : 'text-muted-foreground/50',
								dayEntries.length > 0 && 'hover:bg-accent',
								isToday && 'ring-primary font-semibold ring-2',
								selectedKey === key && 'bg-accent ring-primary ring-2'
							)}
						>
							<span>{day.getUTCDate()}</span>
							<span class="flex h-1.5 gap-0.5" aria-hidden="true">
								{#each dayEntries as entry, index (index)}
									<span
										class={cn(
											'size-1.5 rounded-full',
											ACTION_KINDS[entry.kind].dot,
											entry.scheduled && 'opacity-40'
										)}
									></span>
								{/each}
							</span>
						</button>
					{/each}
				</div>
			{/each}
			<p class="text-muted-foreground mt-1 flex flex-wrap gap-x-4 gap-y-1 text-sm">
				<span class="inline-flex items-center gap-1.5">
					<span class="ring-primary size-3 rounded-sm ring-2"></span>Today
				</span>
				<span class="inline-flex items-center gap-1.5">
					<span class="bg-muted/50 size-3 rounded-sm"></span>Conversation open
				</span>
				<span class="inline-flex items-center gap-1.5">
					<span class="bg-primary size-1.5 rounded-full opacity-40"></span>Planned
				</span>
			</p>
		</div>

		<div class="flex flex-col gap-3">
			<div class="flex items-baseline justify-between gap-3">
				<h3 class="text-base font-semibold">
					{selectedKey ? formatActionDay(new Date(selectedKey)) : 'All activity'}
				</h3>
				{#if selectedKey}
					<button
						type="button"
						class="text-primary text-sm font-semibold hover:underline"
						onclick={() => (selectedKey = null)}
					>
						Show all
					</button>
				{:else}
					<a
						href={resolve('/(admin)/admin/conversations/[conversation_id]/invites', {
							conversation_id: conversationId
						})}
						class="text-primary text-sm font-semibold hover:underline">Add activity</a
					>
				{/if}
			</div>
			<ol class="-mr-2 flex max-h-[24rem] flex-col gap-2 overflow-y-auto pr-2">
				{#each shownEntries as entry (dayKey(entry.date) + entry.title)}
					<li><ActionLogItem {entry} /></li>
				{/each}
			</ol>
		</div>
	</Card.Content>
</Card.Root>
