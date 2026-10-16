<!--
	Fork of $lib/components/StatsBar.svelte with an explanation popover per stat, on a light
	card surface. Used for the live numbers on Live pulse.
	TODO(frontend): fold these into the shared StatsBar, or replace it.
-->
<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Info } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Stat = {
		name: string;
		amount: number;
		unit: string | null;
		change?: string;
		from?: string;
		/** Extra lines under the number, e.g. "100 people recruited". */
		details?: string[];
		/** One plain-English line under the number saying what it counts. */
		meaning?: string;
		/** Longer explanation behind an info button next to the label. */
		explanation?: string;
	};
	type Props = {
		stats: Array<Stat>;
	};

	let { stats }: Props = $props();

	const styles = {
		bar: 'bg-card border',
		divider: 'border-border',
		label: 'text-muted-foreground',
		value: 'text-foreground',
		detail: 'text-muted-foreground',
		infoButton: 'bg-muted text-muted-foreground hover:bg-muted/70'
	};
</script>

<div class={cn('mb-5 flex w-full flex-col gap-5 rounded-3xl p-6', styles.bar)}>
	<div class="grid w-full grid-cols-2 gap-y-6 md:flex md:items-start md:justify-around">
		{#each stats as stat, index (stat.name)}
			<div
				class={cn(
					'inline-flex flex-1 flex-col items-start justify-start gap-1 px-5',
					index > 0 && 'md:border-l',
					styles.divider
				)}
			>
				<div class="inline-flex items-center gap-2 self-stretch">
					<div
						class={cn(
							"text-foreground font-['Inter'] text-sm leading-tight font-medium",
							styles.label
						)}
					>
						{stat.name}
					</div>
					{#if stat.explanation}
						<Popover.Root>
							<Popover.Trigger
								aria-label="What does {stat.name} count?"
								class={cn(
									'inline-flex size-7 items-center justify-center rounded-full',
									styles.infoButton
								)}
							>
								<Info class="size-4" aria-hidden="true" />
							</Popover.Trigger>
							<Popover.Content class="w-72 text-base">
								<p class="font-semibold">{stat.name}</p>
								<p class="text-muted-foreground mt-1">{stat.explanation}</p>
							</Popover.Content>
						</Popover.Root>
					{/if}
				</div>

				<div
					class={cn(
						"self-stretch font-['Inter'] text-3xl leading-9 font-semibold",
						styles.value
					)}
				>
					{stat.amount}
					{stat.unit}
				</div>
				{#if stat.change}
					<div
						class={cn(
							"self-stretch font-['Inter'] text-sm leading-tight font-semibold",
							styles.detail
						)}
					>
						{stat.change}
						{stat.from}
					</div>
				{/if}
				{#if stat.meaning}
					<div
						class={cn("self-stretch font-['Inter'] text-sm leading-snug", styles.label)}
					>
						{stat.meaning}
					</div>
				{/if}
				{#each stat.details ?? [] as detail (detail)}
					<div
						class={cn(
							"self-stretch font-['Inter'] text-sm leading-tight",
							styles.detail
						)}
					>
						{detail}
					</div>
				{/each}
			</div>
		{/each}
	</div>
</div>
