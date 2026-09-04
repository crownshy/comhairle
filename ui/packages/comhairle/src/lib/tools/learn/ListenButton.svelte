<script lang="ts">
	import { Headphones, Pause, Play } from 'lucide-svelte';
	import { listen } from '$lib/components/participant/listen.svelte';
	import { haptic } from '$lib/utils/haptics';
	import * as m from '$lib/paraglide/messages';

	let active = $derived(listen.status !== 'idle');
	let fillPercent = $derived(Math.round(listen.progress * 100));

	function handleClick() {
		haptic('light');
		listen.toggle();
	}
</script>

<!-- The offer, at the top of the page where the reading starts (ADR-0031). It quotes the
     cost before it asks. Once playing it becomes the page's progress track, filling left to
     right block by block, and the transport in the bar takes over as the control that stays
     on screen. -->
<button
	type="button"
	class="bg-accent text-accent-foreground relative flex h-11 w-full items-center gap-3 overflow-hidden rounded-full px-4 text-left text-base font-medium transition-transform active:scale-[0.98] motion-reduce:transition-none motion-reduce:active:scale-100"
	aria-pressed={active}
	onclick={handleClick}
>
	{#if active}
		<span
			aria-hidden="true"
			class="bg-primary/15 absolute inset-y-0 left-0 transition-[width] duration-500 ease-out motion-reduce:transition-none"
			style:width="{fillPercent}%"
		></span>
	{/if}
	<span class="relative flex min-w-0 flex-1 items-center gap-3">
		{#if !active}
			<Headphones class="size-5 shrink-0" />
			<span class="truncate">{m.listen_to_page()}</span>
			<span class="text-muted-foreground ml-auto shrink-0">
				{m.listen_minutes({ count: listen.minutes })}
			</span>
		{:else if listen.status === 'playing'}
			<Pause class="size-5 shrink-0" />
			<span class="truncate">{m.listen_pause()}</span>
			<span class="text-muted-foreground ml-auto shrink-0 tabular-nums">
				{m.listen_progress({ current: listen.index + 1, total: listen.blockCount })}
			</span>
		{:else}
			<Play class="size-5 shrink-0" />
			<span class="truncate">{m.listen_resume()}</span>
			<span class="text-muted-foreground ml-auto shrink-0 tabular-nums">
				{m.listen_progress({ current: listen.index + 1, total: listen.blockCount })}
			</span>
		{/if}
	</span>
</button>
