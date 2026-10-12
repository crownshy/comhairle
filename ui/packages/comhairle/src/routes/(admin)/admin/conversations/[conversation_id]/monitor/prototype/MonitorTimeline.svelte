<script lang="ts">
	import { Check } from 'lucide-svelte';
	import type { RecruitmentEvent } from './sampleMonitorData';

	type TimelineStat = { label: string; value: string; detail: string };

	type Props = {
		/** 'dayOne' is the empty state a new admin sees before anyone joins. */
		state: 'running' | 'dayOne';
		day: number;
		totalDays: number;
		launchDate: Date;
		closingDate: Date;
		events: RecruitmentEvent[];
		activeNow: number;
		stats: TimelineStat[];
		recruitHref: string;
	};

	let {
		state,
		day,
		totalDays,
		launchDate,
		closingDate,
		events,
		activeNow,
		stats,
		recruitHref
	}: Props = $props();

	const DAY_MS = 24 * 60 * 60 * 1000;

	function percentAlong(date: Date): number {
		return Math.min(
			100,
			Math.max(0, ((date.getTime() - launchDate.getTime()) / DAY_MS / totalDays) * 100)
		);
	}

	function formatDay(date: Date): string {
		return date.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' });
	}

	let todayPercent = $derived(
		state === 'dayOne' ? 0 : Math.min(100, ((day - 0.5) / totalDays) * 100)
	);
	let phase = $derived.by(() => {
		if (state === 'dayOne') return 'your conversation is live';
		const share = day / totalDays;
		if (share < 0.4) return 'early days';
		if (share < 0.6) return 'halfway through';
		if (share < 0.9) return 'in the final stretch';
		return 'closing soon';
	});

	const RECRUIT_STEPS = [
		{ label: 'Share an open link', subtab: 'open-links' },
		{ label: 'Send email invites', subtab: 'email' },
		{ label: 'Print QR codes for an event', subtab: 'physical' }
	];
</script>

<!--
	New component: the stat bar anchored on the conversation's timeline. Shows where
	you are between launch and close, the recruitment activity so far, and the numbers.
	The day-one state replaces the numbers with a first-steps checklist.
-->
<section
	aria-label="Key numbers"
	class="bg-card mb-5 flex flex-col gap-7 rounded-3xl border p-6 md:p-7"
>
	<div class="flex flex-wrap items-baseline justify-between gap-3">
		<p class="flex flex-wrap items-baseline gap-2.5">
			<span class="text-3xl font-bold">Day {state === 'dayOne' ? 1 : day}</span>
			<span class="text-muted-foreground text-base">of {totalDays} · {phase}</span>
		</p>
		{#if state === 'running'}
			<span class="inline-flex items-center gap-2 text-sm font-medium">
				<span class="size-2 rounded-full bg-green-600" aria-hidden="true"></span>
				{activeNow} taking part right now
			</span>
		{/if}
	</div>

	<!-- Track -->
	<div class="flex flex-col gap-2">
		{#if state === 'running'}
			<!-- Recruitment activity sits above the track so it never collides with the dates below. -->
			<div class="text-muted-foreground relative h-5 text-sm" aria-hidden="true">
				{#each events as event (event.label)}
					<span
						class="absolute -translate-x-1/2 whitespace-nowrap"
						style:left="max(3rem, {percentAlong(event.day)}%)"
					>
						{event.label}
					</span>
				{/each}
			</div>
		{/if}
		<div
			class="relative h-6"
			role="img"
			aria-label="Day {day} of {totalDays}, from {formatDay(launchDate)} to {formatDay(
				closingDate
			)}"
		>
			<div class="bg-muted absolute inset-x-0 top-2.5 h-1.5 rounded-full"></div>
			<div
				class="bg-primary absolute top-2.5 left-0 h-1.5 rounded-full"
				style:width="{todayPercent}%"
			></div>
			{#if state === 'running'}
				{#each events as event (event.label)}
					<div
						class="border-primary bg-card absolute top-1.5 size-3.5 -translate-x-1/2 rounded-full border-[3px]"
						style:left="{percentAlong(event.day)}%"
					></div>
				{/each}
			{/if}
			<div
				class="bg-primary ring-accent absolute top-0.5 size-5 -translate-x-1/2 rounded-full ring-4"
				style:left="{Math.max(todayPercent, 1.5)}%"
			></div>
			<div
				class="border-muted-foreground bg-card absolute top-1 right-0 size-4 rounded-sm border-2"
			></div>
		</div>
		<div class="text-muted-foreground relative h-10 text-sm">
			<span class="absolute left-0">Launched<br />{formatDay(launchDate)}</span>
			{#if state === 'running'}
				<span
					class="text-foreground absolute -translate-x-1/2 font-semibold"
					style:left="{todayPercent}%">Today</span
				>
			{/if}
			<span class="absolute right-0 text-right">Closes<br />{formatDay(closingDate)}</span>
		</div>
	</div>

	{#if state === 'running'}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
			{#each stats as stat (stat.label)}
				<div class="bg-muted/60 flex flex-col gap-1 rounded-2xl px-5 py-4">
					<span class="text-muted-foreground text-sm font-semibold">{stat.label}</span>
					<span class="text-3xl font-bold">{stat.value}</span>
					<span class="text-muted-foreground text-sm">{stat.detail}</span>
				</div>
			{/each}
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-6 md:grid-cols-[minmax(0,1fr)_minmax(0,1.4fr)]">
			<div class="grid grid-cols-2 content-start gap-3">
				{#each ['Joined', 'Finished'] as label (label)}
					<div class="bg-muted/60 flex flex-col gap-1 rounded-2xl px-5 py-4">
						<span class="text-muted-foreground text-sm font-semibold">{label}</span>
						<span class="text-muted-foreground text-3xl font-bold">0</span>
					</div>
				{/each}
				<p class="text-muted-foreground col-span-2 text-base">
					Numbers appear here as soon as the first person joins. Most conversations see
					their first participants within a day of sharing a link.
				</p>
			</div>
			<div class="bg-accent flex flex-col gap-3 rounded-2xl p-5">
				<h3 class="text-lg font-semibold">Get your first participants</h3>
				<ol class="flex flex-col gap-2.5">
					<li class="flex items-center gap-3 text-base">
						<span
							class="bg-primary text-primary-foreground inline-flex size-7 items-center justify-center rounded-full"
						>
							<Check class="size-4" aria-hidden="true" />
						</span>
						<span class="text-muted-foreground line-through"
							>Launch the conversation</span
						>
						<span class="sr-only">(done)</span>
					</li>
					{#each RECRUIT_STEPS as step, index (step.label)}
						<li class="flex items-center gap-3 text-base">
							<span
								class="border-primary text-primary inline-flex size-7 items-center justify-center rounded-full border-2 text-sm font-bold"
							>
								{index + 2}
							</span>
							<a
								href="{recruitHref}?subtab={step.subtab}"
								class="text-primary font-semibold hover:underline"
							>
								{step.label}
							</a>
						</li>
					{/each}
				</ol>
			</div>
		</div>
	{/if}
</section>
