<script lang="ts">
	import { Sparkles } from 'lucide-svelte';
	import { m } from '$lib/paraglide/messages';
	import { getLocale } from '$lib/paraglide/runtime';
	import { participationSummary } from './participationSummary';

	type Props = {
		participantCount: number;
	};
	let { participantCount }: Props = $props();

	let summary = $derived(participationSummary(participantCount, getLocale()));
</script>

<div class="bg-card flex items-center gap-4 rounded-lg p-5">
	{#if summary.kind === 'count'}
		<p class="flex items-center gap-4 text-base md:text-lg">
			<span class="text-primary text-4xl leading-none font-bold tabular-nums md:text-5xl">
				{summary.formattedCount}
			</span>
			<span class="flex flex-col gap-1">
				<span class="font-bold">{m.thank_you_people_taken_part()}</span>
				<span>{m.thank_you_including_you()}</span>
			</span>
		</p>
	{:else}
		<Sparkles class="text-primary size-10 shrink-0" aria-hidden="true" />
		<p class="flex flex-col gap-1 text-base md:text-lg">
			<span class="font-bold">{m.thank_you_early_participant()}</span>
			<span>{m.thank_you_early_participant_body()}</span>
		</p>
	{/if}
</div>
