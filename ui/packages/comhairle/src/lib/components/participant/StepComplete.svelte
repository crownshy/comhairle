<script lang="ts">
	import { Check } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { haptic } from '$lib/utils/haptics';
	import * as m from '$lib/paraglide/messages';

	// The screen only ever appears after a tap, so the buzz lands on the tap that earned it.
	onMount(() => {
		haptic('success');
	});
</script>

<!-- The beat between finishing a step and moving on: the tool is gone, the progress segment
	is full, and the only thing left to do is proceed. Announced politely because a
	participant who submitted with the keyboard never sees the swap. -->
<section
	class="flex w-full grow flex-col items-center justify-center gap-12 px-6 py-8 text-center"
	role="status"
	aria-live="polite"
>
	<h2
		class="animate-in fade-in slide-in-from-bottom-2 text-3xl font-bold duration-500 motion-reduce:animate-none"
	>
		{m.step_complete_title()}
	</h2>
	<!-- The disc pops in and overshoots a touch, then the tick lands inside it a beat later.
		Two entrances rather than one so the eye reads "done" twice. -->
	<div
		class="bg-step-complete text-step-complete-foreground step-complete-pop flex size-40 items-center justify-center rounded-full motion-reduce:animate-none md:size-48"
		aria-hidden="true"
	>
		<Check
			class="animate-in zoom-in-50 fade-in fill-mode-both size-20 delay-200 duration-400 ease-out motion-reduce:animate-none md:size-24"
			strokeWidth={3}
		/>
	</div>
</section>

<style>
	.step-complete-pop {
		animation: step-complete-pop 600ms cubic-bezier(0.34, 1.56, 0.64, 1) both;
	}

	@keyframes step-complete-pop {
		from {
			opacity: 0;
			transform: scale(0.4);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}
</style>
