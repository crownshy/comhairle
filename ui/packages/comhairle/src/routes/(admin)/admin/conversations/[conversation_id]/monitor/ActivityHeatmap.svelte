<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { cn } from '$lib/utils';

	type Props = {
		days: string[];
		slots: string[];
		/** counts[dayIndex][slotIndex] */
		counts: number[][];
	};

	let { days, slots, counts }: Props = $props();

	let busiest = $derived.by(() => {
		let best = { day: 0, slot: 0, count: -1 };
		counts.forEach((row, day) =>
			row.forEach((count, slot) => {
				if (count > best.count) best = { day, slot, count };
			})
		);
		return best;
	});

	function shadeClass(count: number): string {
		if (count >= 25) return 'bg-primary text-primary-foreground';
		if (count >= 12) return 'bg-primary/70 text-primary-foreground';
		if (count >= 7) return 'bg-primary/35';
		if (count >= 3) return 'bg-primary/15';
		return 'bg-muted text-muted-foreground';
	}
</script>

<!-- When participants are active, by weekday and time of day, to help time reminders. -->
<Card.Root>
	<Card.Header>
		<Card.Title>When people take part</Card.Title>
		<Card.Description class="text-base">
			Busiest: <strong class="text-foreground"
				>{days[busiest.day]}, {slots[busiest.slot]}</strong
			>.
		</Card.Description>
	</Card.Header>
	<Card.Content class="overflow-x-auto">
		<!-- Days across the top, times of day down the side. counts stays [day][slot]. -->
		<table class="w-full min-w-[480px] border-separate border-spacing-1 text-sm">
			<thead>
				<tr>
					<th class="w-20"><span class="sr-only">Time of day</span></th>
					{#each days as day (day)}
						<th scope="col" class="text-muted-foreground font-normal">{day}</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each slots as slot, slotIndex (slot)}
					<tr>
						<th scope="row" class="text-left font-normal whitespace-nowrap">{slot}</th>
						{#each days as day, dayIndex (day)}
							{@const count = counts[dayIndex][slotIndex]}
							<td class={cn('h-9 rounded text-center', shadeClass(count))}>{count}</td
							>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</Card.Content>
	<Card.Footer class="text-muted-foreground text-sm">
		Actions taken (step starts, answers, comments) in each slot since launch.
	</Card.Footer>
</Card.Root>
