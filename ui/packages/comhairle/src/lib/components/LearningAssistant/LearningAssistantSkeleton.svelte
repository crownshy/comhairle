<script lang="ts">
	/**
	 * Placeholder for the whole assistant panel, for when none of it has mounted yet (the
	 * step page renders this while it routes). Once the panel itself is up the ask bar is
	 * real and only the answer is missing, so that case renders AssistantAnswerSkeleton.
	 */
	import { Search, ArrowUp } from 'lucide-svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import * as m from '$lib/paraglide/messages';
	import AssistantAnswerSkeleton from './AssistantAnswerSkeleton.svelte';
</script>

<section class="my-6 flex flex-col gap-6" aria-busy="true" aria-live="polite">
	<div>
		<h2 class="text-foreground mb-2 text-lg font-bold">{m.learning_assistant()}</h2>
		<div
			class="border-input bg-background flex items-center gap-3 rounded-2xl border-2 px-4 py-3"
		>
			<Search class="text-muted-foreground h-5 w-5 shrink-0" />
			<Skeleton class="h-5 w-56 max-w-full" />
			<div
				class="bg-primary/40 ml-auto grid h-10 w-10 shrink-0 place-items-center rounded-xl"
			>
				<ArrowUp class="text-primary-foreground h-5 w-5" />
			</div>
		</div>
	</div>

	<AssistantAnswerSkeleton />

	<span class="sr-only">Loading learning assistant</span>
</section>
