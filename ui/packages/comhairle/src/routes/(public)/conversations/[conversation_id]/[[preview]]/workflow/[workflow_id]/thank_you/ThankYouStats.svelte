<script lang="ts">
	/**
	 * What the participant just did, as numbers that count themselves up.
	 *
	 * Every figure is measured rather than estimated: the steps and the percentage come from
	 * this participant's progress, the minutes from the clock the step pages stamp. Minutes
	 * are optional, because that clock lives in the browser and can be missing (see
	 * flowTiming).
	 */
	import CountUp from './CountUp.svelte';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';

	let {
		minutes,
		stepsDone,
		percentComplete
	}: {
		minutes: number | null;
		stepsDone: number;
		percentComplete: number;
	} = $props();

	type Tile = { key: string; value: number; suffix?: string; label: string };

	let tiles = $derived.by<Tile[]>(() => {
		const rows: Tile[] = [];
		if (minutes !== null) {
			rows.push({
				key: 'minutes',
				value: minutes,
				// The space belongs to the layout, not to the translation.
				suffix: ` ${m.thank_you_stat_minutes_suffix()}`,
				label: m.thank_you_stat_time()
			});
		}
		rows.push({ key: 'steps', value: stepsDone, label: m.thank_you_stat_steps() });
		rows.push({
			key: 'percent',
			value: percentComplete,
			suffix: '%',
			label: m.thank_you_stat_completed()
		});
		return rows;
	});
</script>

<dl class="grid w-full grid-cols-2 gap-3 sm:grid-cols-3">
	{#each tiles as tile, index (tile.key)}
		<!-- Staggered so the row lands a tile at a time, in the order the numbers start
			counting. The label is the term and the number its value, so the pair is written
			that way round and the column is reversed to put the number on top. -->
		<div
			class={cn(
				'bg-accent animate-in fade-in slide-in-from-bottom-2 fill-mode-both flex flex-col-reverse items-center gap-1 rounded-2xl px-4 py-5 duration-500 motion-reduce:animate-none',
				// An odd tile out would otherwise sit half-width beside a gap on a phone.
				tiles.length % 2 === 1 && index === tiles.length - 1 && 'max-sm:col-span-2'
			)}
			style="animation-delay: {index * 120}ms"
		>
			<dt class="text-muted-foreground text-sm">{tile.label}</dt>
			<dd class="text-primary text-3xl font-bold tabular-nums md:text-4xl">
				<CountUp value={tile.value} delay={index * 120} />{tile.suffix ?? ''}
			</dd>
		</div>
	{/each}
</dl>
