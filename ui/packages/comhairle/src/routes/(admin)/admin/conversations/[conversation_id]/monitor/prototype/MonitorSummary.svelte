<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Sparkles } from 'lucide-svelte';

	type Props = {
		dayOfConversation: number;
		conversationLengthDays: number;
		joined: number;
		finished: number;
		biggestDropOffStep: string;
		notifyHref: string;
		stepHref: string;
	};

	let {
		dayOfConversation,
		conversationLengthDays,
		joined,
		finished,
		biggestDropOffStep,
		notifyHref,
		stepHref
	}: Props = $props();
</script>

<!-- AI mode: plain-language verdict. Reuses the StatsBar surface (bg-sidebar, rounded-3xl). -->
<section
	aria-label="Summary"
	class="bg-sidebar text-sidebar-foreground flex flex-wrap items-center justify-between gap-6 rounded-3xl p-6 md:p-8"
>
	<div class="flex min-w-0 flex-[1_1_32rem] flex-col gap-2">
		<span
			class="text-sidebar-secondary inline-flex items-center gap-1.5 text-sm font-semibold tracking-wide uppercase"
		>
			<Sparkles class="size-4" aria-hidden="true" />
			AI summary · Day {dayOfConversation} of {conversationLengthDays}
		</span>
		<p class="text-xl leading-relaxed font-medium md:text-2xl">
			You're on track: <strong>{joined} people</strong> have joined and
			<strong>{finished}</strong> have finished. Most people who drop out leave at
			<strong>{biggestDropOffStep}</strong>.
		</p>
	</div>
	<div class="flex flex-wrap gap-3">
		<Button href={notifyHref} size="lg" class="rounded-full">Remind people who stopped</Button>
		<Button href={stepHref} variant="nav" size="lg" class="rounded-full border border-white/30">
			Look at {biggestDropOffStep}
		</Button>
	</div>
</section>
