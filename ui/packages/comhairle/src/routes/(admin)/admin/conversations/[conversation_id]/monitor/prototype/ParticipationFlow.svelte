<script lang="ts">
	import { ArrowDown, ArrowRight } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Props = {
		/** Signed up through links, emails and QR codes. */
		recruited: number;
		/** Opened the conversation. */
		joined: number;
		/** Completed every step. */
		completed: number;
		/** Line across the top right of the panel, e.g. "Day 14 of 28 · closes 26 Oct". */
		caption?: string;
	};

	let { recruited, joined, completed, caption }: Props = $props();

	function percentOf(value: number, total: number): number {
		return total === 0 ? 0 : Math.round((value / total) * 100);
	}

	let stages = $derived([
		{
			label: 'Recruited',
			value: recruited,
			note: 'Signed up through links, emails and QR codes'
		},
		{
			label: 'Joined',
			value: joined,
			note: `Opened the conversation · ${joined - completed} still part-way`
		},
		{ label: 'Completed', value: completed, note: 'Completed every step' }
	]);

	/** Share of the previous stage that carried on, shown on the arrows. */
	let rates = $derived([
		`${percentOf(joined, recruited)}% joined`,
		`${percentOf(completed, joined)}% completed`
	]);
</script>

<!--
	The At a glance headline: Recruited → Joined → Completed on the brand surface, with the
	share that carried on at each arrow. Completed is the white card so the outcome stands out.
-->
<section
	aria-labelledby="participation-flow-heading"
	class="bg-primary text-primary-foreground flex flex-col gap-5 rounded-3xl px-7 pt-6 pb-7"
>
	<div class="flex flex-wrap items-baseline justify-between gap-3">
		<h2 id="participation-flow-heading" class="text-base font-semibold">
			How people move through your conversation
		</h2>
		{#if caption}
			<span class="text-accent text-sm">{caption}</span>
		{/if}
	</div>

	<ol
		class="grid grid-cols-1 gap-2 md:grid-cols-[minmax(0,1fr)_8rem_minmax(0,1fr)_8rem_minmax(0,1fr)] md:gap-0"
	>
		{#each stages as stage, index (stage.label)}
			{@const last = index === stages.length - 1}
			<li
				class={cn(
					'flex flex-col gap-1.5 rounded-2xl px-5 py-4',
					last ? 'bg-card text-card-foreground' : 'bg-primary-foreground/10'
				)}
			>
				<span
					class={cn(
						'text-sm font-semibold',
						last ? 'text-muted-foreground' : 'text-accent'
					)}>{stage.label}</span
				>
				<span class="text-4xl leading-tight font-bold">{stage.value}</span>
				<span
					class={cn(
						'text-sm leading-snug',
						last ? 'text-muted-foreground' : 'text-accent'
					)}>{stage.note}</span
				>
			</li>
			{#if !last}
				<li
					class="flex items-center justify-center gap-2 py-1 md:flex-col md:py-0"
					aria-label={rates[index]}
				>
					<ArrowRight class="hidden size-8 md:block" aria-hidden="true" />
					<ArrowDown class="size-5 md:hidden" aria-hidden="true" />
					<span
						class="bg-card text-primary rounded-full px-2.5 py-0.5 text-sm font-semibold"
						>{rates[index]}</span
					>
				</li>
			{/if}
		{/each}
	</ol>
</section>
