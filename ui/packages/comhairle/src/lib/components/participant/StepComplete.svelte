<script lang="ts">
	import { Check } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
</script>

<!-- The beat between finishing a step and moving on: the tool is gone, the progress segment
	is full, and the only thing left to do is proceed. Announced politely because a
	participant who submitted with the keyboard never sees the swap. -->
<section
	class="flex w-full grow flex-col items-center justify-center gap-12 px-6 py-8 text-center"
	role="status"
	aria-live="polite"
>
	<h2 class="text-3xl font-bold">{m.step_complete_title()}</h2>
	<div
		class="bg-step-complete text-step-complete-foreground flex size-40 items-center justify-center rounded-full md:size-48"
		aria-hidden="true"
	>
		<Check class="size-20 md:size-24" strokeWidth={3} />
	</div>
</section>
