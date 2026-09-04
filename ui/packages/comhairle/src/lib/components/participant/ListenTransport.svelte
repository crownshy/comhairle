<script lang="ts">
	import { Pause, Play } from 'lucide-svelte';
	import { listen } from './listen.svelte';
	import { haptic } from '$lib/utils/haptics';
	import * as m from '$lib/paraglide/messages';

	let playing = $derived(listen.status === 'playing');
	let rateLabel = $derived(`${listen.rate}×`);

	function handleToggle() {
		haptic('light');
		listen.toggle();
	}

	function handleRate() {
		haptic('light');
		listen.cycleRate();
	}
</script>

<!-- The controls a listener needs while reading along, in the middle of the bar between
     Back and Next so they stay on screen for the whole page (ADR-0031). Pause and speed
     and nothing else: the offer at the top of the page already says what is playing. -->
<div class="flex items-center gap-2">
	<button
		type="button"
		class="bg-foreground text-background inline-flex size-11 shrink-0 items-center justify-center rounded-full transition-transform active:scale-90 motion-reduce:transition-none motion-reduce:active:scale-100"
		aria-label={playing ? m.listen_pause() : m.listen_play()}
		onclick={handleToggle}
	>
		{#if playing}
			<Pause class="size-5" />
		{:else}
			<Play class="size-5 translate-x-px" />
		{/if}
	</button>
	<button
		type="button"
		class="bg-accent text-accent-foreground inline-flex h-9 min-w-14 shrink-0 items-center justify-center rounded-full px-3 text-base font-medium tabular-nums transition-transform active:scale-90 motion-reduce:transition-none motion-reduce:active:scale-100"
		aria-label={m.listen_speed({ rate: rateLabel })}
		onclick={handleRate}
	>
		{rateLabel}
	</button>
</div>
