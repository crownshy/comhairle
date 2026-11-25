<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import { m } from '$lib/paraglide/messages';
	import { focusOnScreenChange } from '$lib/utils/focusOnScreenChange';
</script>

<section class="flex w-full grow flex-col items-center justify-center gap-12 px-6 py-8 text-center">
	<h2
		tabindex="-1"
		{@attach focusOnScreenChange}
		class="animate-in fade-in slide-in-from-bottom-2 text-3xl font-bold duration-500 outline-none motion-reduce:animate-none"
	>
		{m.step_complete_title()}
	</h2>
	<div
		class="bg-step-complete text-step-complete-foreground animate-in zoom-in-50 fade-in flex size-40 items-center justify-center rounded-full duration-500 ease-out motion-reduce:animate-none md:size-48"
		aria-hidden="true"
	>
		<Check
			class="animate-in zoom-in-50 fade-in fill-mode-both size-20 delay-200 duration-400 ease-out motion-reduce:animate-none md:size-24"
			strokeWidth={3}
		/>
	</div>
</section>
